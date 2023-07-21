use std::{cell::RefCell, collections::VecDeque, fmt, rc::Rc};

use ahash::{HashMap, HashMapExt};
use dprint_swc_ext::view::Node;

#[derive(Debug, Default)]
struct Scope<'a> {
    symbols: HashMap<String, Declaration<'a>>,
    children: Option<Vec<Rc<RefCell<Scope<'a>>>>>,
    parent: Option<Rc<RefCell<Scope<'a>>>>,
}

#[derive(Debug, Default)]
pub struct DeclarationTables<'n> {
    tables: HashMap<String, DeclarationTable<'n>>,
}
impl<'n> DeclarationTables<'n> {
    pub fn has_table(&self, file_name: &str) -> bool {
        self.tables.contains_key(file_name)
    }

    pub fn insert(&mut self, file_path: &str, name: String, value: Declaration<'n>) -> () {
        let table = self.tables.entry(file_path.to_owned()).or_insert_with(Default::default);
        table.insert(name, value);
    }

    pub fn add_child_scope(&mut self, file_path: &str) -> () {
        self.tables
            .entry(file_path.to_owned())
            .or_insert_with(Default::default)
            .add_child_scope();
    }

    pub fn parent_scope(&mut self, file_path: &str) -> () {
        self.tables
            .entry(file_path.to_owned())
            .or_insert_with(Default::default)
            .parent_scope();
    }

    pub fn get_root_declaration_name(&mut self, file_path: &str, reference: String) -> String {
        self.tables
            .entry(file_path.to_owned())
            .or_insert_with(Default::default)
            .get_root_declaration_name(reference)
    }

    pub(crate) fn get_root_declaration(&mut self, file_path: &str, type_reference: &str) -> Option<Declaration<'n>> {
        self.tables
            .entry(file_path.to_owned())
            .or_insert_with(Default::default)
            .get_root_declaration(type_reference)
    }
}

#[derive(Debug, Default)]
pub struct DeclarationTable<'a> {
    current_scope: Rc<RefCell<Scope<'a>>>,
    root_scope: Rc<RefCell<Scope<'a>>>,
}
impl<'a> DeclarationTable<'a> {
    pub(crate) fn new() -> DeclarationTable<'a> {
        let root_scope = Rc::new(RefCell::new(Scope {
            symbols: HashMap::new(),
            children: None,
            parent: None,
        }));

        DeclarationTable {
            current_scope: Rc::clone(&root_scope),
            root_scope,
        }
    }

    fn insert(&mut self, name: String, value: Declaration<'a>) -> () {
        self.current_scope.borrow_mut().symbols.insert(name, value);
    }

    fn add_child_scope(&mut self) -> &mut DeclarationTable<'a> {
        let child_scope = Rc::new(RefCell::new(Scope {
            symbols: HashMap::new(),
            children: None,
            parent: Some(Rc::clone(&self.current_scope)),
        }));

        self.current_scope
            .borrow_mut()
            .children
            .get_or_insert_with(Default::default)
            .push(Rc::clone(&child_scope));

        self.current_scope = child_scope;

        self
    }

    fn parent_scope(&mut self) -> &mut DeclarationTable<'a> {
        let parent = Rc::clone(
            self.current_scope
                .borrow()
                .parent
                .as_ref()
                .expect("Expected current scope to have parent."),
        );

        self.current_scope = Rc::clone(&parent);

        self
    }

    fn get_root_declaration(&self, reference: &str) -> Option<Declaration<'a>> {
        let mut declaration: Option<Declaration> = None;
        let mut queue = VecDeque::from([Rc::clone(&self.current_scope)]);
        let mut references = vec![reference.to_string()];
        let mut last_reference = String::from("__none__");

        while declaration.is_none() && !queue.is_empty() {
            if let Some(scope) = queue.pop_front() {
                let scope = scope.borrow();
                while references.len() > 0 {
                    let temp = references.pop().unwrap();
                    match scope.symbols.get(&temp){
                        Some(Declaration::Alias { from: _, to }) => {
                            references.push(to.clone());
                        },
                        _ => {}
                    }
                    last_reference = temp;
                }

                match scope.symbols.get(&last_reference){
                    Some(decl) => declaration = Some(decl.clone()),
                    None => if let Some(parent) = &scope.parent {
                        queue.push_back(Rc::clone(&parent));
                    },
                }
            }
        }

        return declaration;
    }

    fn get_root_declaration_name(&self, reference: String) -> String {
        let mut current = reference.clone();
        let mut previous = reference.clone();
        let mut queue = VecDeque::from([Rc::clone(&self.current_scope)]);
        while !queue.is_empty() {
            if let Some(scope) = queue.pop_front() {
                let scope = scope.borrow();
                match scope.symbols.get(&current) {
                    Some(Declaration::Alias { from: _, to }) => current = to.clone(),
                    Some(Declaration::Export {
                        name,
                        source_file_name: _,
                    }) => current = name.clone(),
                    Some(Declaration::Import {
                        name,
                        source_file_name: _,
                    }) => current = name.clone(),
                    _ => {}
                };

                if previous.eq(&current) {
                    break;
                }

                previous = current.clone();

                if let Some(parent) = &scope.parent {
                    queue.push_back(Rc::clone(&parent))
                }
            }
        }

        current.to_owned()
    }

    fn get_declaration(&self, type_reference: &str) -> Option<Declaration<'a>> {
        let mut queue = VecDeque::from([Rc::clone(&self.current_scope)]);
        while !queue.is_empty() {
            if let Some(scope) = queue.pop_front() {
                let scope = scope.borrow();
                if let Some(declaration) = scope.symbols.get(type_reference) {
                    return Some(Declaration::clone(declaration));
                }

                if let Some(parent) = &scope.parent {
                    queue.push_back(Rc::clone(&parent))
                }
            }
        }

        None
    }
}

pub enum Declaration<'a> {
    Alias { from: String, to: String },
    Type { node: Node<'a> },
    Export { name: String, source_file_name: String },
    Import { name: String, source_file_name: String },
}

impl<'n> Clone for Declaration<'n> {
    fn clone(&self) -> Self {
        match self {
            Self::Alias { from, to } => Self::Alias {
                from: from.clone(),
                to: to.clone(),
            },
            Self::Type { node } => Self::Type { node: node.clone() },
            Self::Export { name, source_file_name } => Self::Export {
                name: name.clone(),
                source_file_name: source_file_name.clone(),
            },
            Self::Import { name, source_file_name } => Self::Import {
                name: name.clone(),
                source_file_name: source_file_name.clone(),
            },
        }
    }
}

impl<'n> fmt::Debug for Declaration<'n> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Alias { from, to } => f.debug_struct("Alias").field("from", from).field("to", to).finish(),
            Self::Export { name, source_file_name } => f
                .debug_struct("Export")
                .field("name", name)
                .field("source_file_name", source_file_name)
                .finish(),
            Self::Import { name, source_file_name } => f
                .debug_struct("Import")
                .field("name", name)
                .field("source_file_name", source_file_name)
                .finish(),
            _ => fmt::Result::Ok(()),
        }
    }
}

// impl<'n> Serialize for Declaration<'n> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         serializer.serialize_none()
//     }
// }
