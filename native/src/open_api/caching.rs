use std::{path::PathBuf, rc::Rc};

use es_resolve::{EsResolver, TargetEnv};
use swc_ecma_ast::{Callee, ExportSpecifier, Expr, ImportSpecifier, ModuleExportName, Pat, TsEntityName, TsType};

use crate::typescript::{Declaration, DeclarationTables, NodeKind, SchemyNode};

pub(in crate::open_api) fn store_declaration_maybe(
    root: Rc<SchemyNode>,
    file_path: &str,
    symbol_tables: &mut DeclarationTables,
) -> () {
    match root.kind {
        NodeKind::ModuleItem(_) => {
            for child_item in root.children() {
                store_declaration_maybe(root.get(child_item).unwrap(), file_path, symbol_tables)
            }
        }
        NodeKind::ExportDecl(_) => {
            for child_index in root.children() {
                let child = root.get(child_index).unwrap();
                store_declaration_maybe(child, file_path, symbol_tables)
            }
        }
        NodeKind::ExportDefaultExpr(_) => {
            for child_index in root.children() {
                let child = root.get(child_index).unwrap();
                store_default_declaration(child, file_path, symbol_tables)
            }
        }
        NodeKind::Decl(_) => {
            for child_index in root.children() {
                let child = root.get(child_index).unwrap();
                store_declaration_maybe(child, file_path, symbol_tables)
            }
        }
        NodeKind::ClassDecl(raw) => {
            let name = raw.ident.sym.to_string();
            symbol_tables.insert(
                file_path,
                name.to_string(),
                Declaration::Type {
                    node: root.index.clone(),
                },
            )
        }
        NodeKind::TsInterfaceDecl(raw) => {
            let name = raw.id.sym.to_string();
            symbol_tables.insert(
                file_path,
                name,
                Declaration::Type {
                    node: root.index.clone(),
                },
            )
        }
        NodeKind::TsTypeAliasDecl(raw) => {
            let name = raw.id.sym.to_string();
            symbol_tables.insert(
                file_path,
                name,
                Declaration::Type {
                    node: root.index.clone(),
                },
            )
        }
        NodeKind::TsEnumDecl(raw) => {
            let name = raw.id.sym.to_string();
            symbol_tables.insert(
                file_path,
                name,
                Declaration::Type {
                    node: root.index.clone(),
                },
            )
        }
        NodeKind::ExportDefaultDecl(raw_decl) => {
            match &raw_decl.decl {
                swc_ecma_ast::DefaultDecl::Class(raw_class) => {
                    let class = root.to_child(NodeKind::Class(&*raw_class.class));

                    symbol_tables.insert(
                        file_path,
                        "default".into(),
                        Declaration::Type {
                            node: class.index.clone(),
                        },
                    )
                }
                swc_ecma_ast::DefaultDecl::TsInterfaceDecl(raw_int) => symbol_tables.insert(
                    file_path,
                    "default".into(),
                    Declaration::Alias {
                        from: "default".into(),
                        to: raw_int.id.sym.to_string(),
                    },
                ),
                _ => {}
            };
        }
        NodeKind::ImportDecl(raw) => {
            for child_index in root.children() {
                let child = root.get(child_index).unwrap();
                match child.kind {
                    NodeKind::ImportSpecifier(ImportSpecifier::Default(raw_specifier)) => {
                        let src = raw.src.value.to_string();
                        match EsResolver::new(&src, &PathBuf::from(file_path), TargetEnv::Node).resolve() {
                            Ok(module_path) => {
                                let name = raw_specifier.local.sym.to_string();
                                symbol_tables.insert(
                                    file_path,
                                    name,
                                    Declaration::Import {
                                        name: String::from("default"),
                                        source_file_name: module_path.replace(".js", ".d.ts"),
                                    },
                                )
                            }
                            Err(_) => {}, // TODO improve debugging
                        }
                    }
                    NodeKind::ImportSpecifier(ImportSpecifier::Named(raw_specifier)) => {
                        let src = raw.src.value.to_string();
                        match EsResolver::new(&src, &PathBuf::from(file_path), TargetEnv::Node).resolve() {
                            Ok(module_path) => {
                                let name = &raw_specifier.local.sym;
                                symbol_tables.insert(
                                    file_path,
                                    name.to_string(),
                                    Declaration::Import {
                                        name: name.to_string(),
                                        source_file_name: module_path.replace(".js", ".d.ts"),
                                    },
                                )
                            }
                            Err(_) => {}, // TODO improve debugging
                        }
                    }
                    _ => {}
                }
            }
        }
        NodeKind::NamedExport(raw) => {
            match &raw.src.as_ref() {
                Some(src) => {
                    let src = &src.value;
                    match EsResolver::new(&src, &PathBuf::from(file_path), TargetEnv::Node).resolve() {
                        Ok(module_path) => {
                            for specifier in &raw.specifiers {
                                match specifier {
                                    ExportSpecifier::Named(named_specifier) => {
                                        let type_name = match &named_specifier.orig {
                                            ModuleExportName::Ident(identifier) => &identifier.sym,
                                            ModuleExportName::Str(identifier) => &identifier.value,
                                        };
        
                                        if let Some(exported_name) = &named_specifier.exported {
                                            let exported_name = match exported_name {
                                                ModuleExportName::Ident(id) => &id.sym,
                                                ModuleExportName::Str(id) => &id.value,
                                            };
        
                                            symbol_tables.insert(
                                                file_path,
                                                exported_name.to_string(),
                                                Declaration::Import {
                                                    name: type_name.to_string(),
                                                    source_file_name: module_path.replace(".js", ".d.ts"),
                                                },
                                            )
                                        } else {
                                            symbol_tables.insert(
                                                file_path,
                                                type_name.to_string(),
                                                Declaration::Import {
                                                    name: type_name.to_string(),
                                                    source_file_name: module_path.replace(".js", ".d.ts"),
                                                },
                                            )
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        Err(_) => {}, // TODO improve debugging
                    }
                },
                None => {}
            }
        }
        NodeKind::VarDeclarator(raw) => {
            match &raw.name {
                Pat::Ident(identifier) => {
                    let name = identifier.id.sym.to_string();
                    match &identifier.type_ann {
                        Some(type_annotation) => match &*type_annotation.type_ann {
                            TsType::TsTypeRef(type_ref) => match &type_ref.type_name {
                                TsEntityName::Ident(identifier) => symbol_tables.insert(
                                    file_path,
                                    name.to_string(),
                                    Declaration::Alias {
                                        from: name,
                                        to: identifier.sym.to_string(),
                                    },
                                ),
                                _ => {}
                            },
                            TsType::TsTypeLit(raw_type) => {
                                symbol_tables.insert(
                                    file_path,
                                    name.to_string(),
                                    Declaration::Type {
                                        node: root.to_child(NodeKind::TsTypeLit(raw_type)).index.clone(),
                                    },
                                )
                            }
                            _ => {}
                        },
                        None => match &raw.init {
                            Some(initializer) => {
                                let node = root.to_child(NodeKind::Expr(initializer));
                                store_variable(&name, node, file_path, symbol_tables);
                            }
                            None => {}
                        },
                    }
                }
                _ => {}
            };
        }
        _ => {}
    }
}

fn store_default_declaration(root: Rc<SchemyNode>, file_path: &str, symbol_tables: &mut DeclarationTables) -> () {
    match root.kind {
        NodeKind::CallExpr(raw_call) => match &raw_call.callee {
            Callee::Expr(raw_callee) => match &**raw_callee {
                Expr::Ident(raw_ident) => symbol_tables.insert(
                    file_path,
                    "default".into(),
                    Declaration::Alias {
                        from: "default".into(),
                        to: raw_ident.sym.to_string(),
                    },
                ),
                _ => {}
            },
            _ => {}
        },
        NodeKind::ArrayLit(_) => symbol_tables.insert(
            file_path,
            "default".into(),
            Declaration::Type {
                node: root.index.clone(),
            },
        ),
        NodeKind::ObjectLit(_) => symbol_tables.insert(
            file_path,
            "default".into(),
            Declaration::Type {
                node: root.index.clone(),
            },
        ),
        NodeKind::NewExpr(expr) => match &*expr.callee {
            Expr::Ident(raw_ident) => symbol_tables.insert(
                file_path,
                "default".into(),
                Declaration::Alias {
                    from: "default".into(),
                    to: raw_ident.sym.to_string(),
                },
            ),
            _ => {}
        },
        NodeKind::Ident(raw_ident) => symbol_tables.insert(
            file_path,
            "default".into(),
            Declaration::Alias {
                from: "default".into(),
                to: raw_ident.sym.to_string(),
            },
        ),
        NodeKind::ArrowExpr(_) => {
            symbol_tables.insert(file_path, "default".into(), Declaration::Type { node: root.index })
        }
        NodeKind::ClassExpr(expr) => match &expr.ident {
            Some(raw_ident) => symbol_tables.insert(
                file_path,
                "default".into(),
                Declaration::Alias {
                    from: "default".into(),
                    to: raw_ident.sym.to_string(),
                },
            ),
            None => {}
        },
        NodeKind::TsAsExpr(raw_expr) => match &*raw_expr.type_ann {
            TsType::TsTypeRef(raw_ref) => match &raw_ref.type_name {
                TsEntityName::Ident(raw_ident) => symbol_tables.insert(
                    file_path,
                    "default".into(),
                    Declaration::Alias {
                        from: "default".into(),
                        to: raw_ident.sym.to_string(),
                    },
                ),
                _ => {}
            },
            _ => {}
        },
        NodeKind::TsInstantiationExpr(raw_expr) => match &*raw_expr.expr {
            Expr::Ident(raw_ident) => symbol_tables.insert(
                file_path,
                "default".into(),
                Declaration::Alias {
                    from: "default".into(),
                    to: raw_ident.sym.to_string(),
                },
            ),
            _ => {}
        },
        _ => {}
    }
}

fn store_variable(name: &str, root: Rc<SchemyNode>, file_path: &str, symbol_tables: &mut DeclarationTables) -> () {
    for child_index in root.children() {
        let child = root.get(child_index).unwrap();
        match child.kind {
            NodeKind::Ident(raw) => {
                let type_name = raw.sym.to_string();
                symbol_tables.insert(
                    file_path,
                    name.to_string(),
                    Declaration::Alias {
                        from: name.to_string(),
                        to: type_name,
                    },
                )
            }
            NodeKind::TsTypeRef(raw) => match &raw.type_name {
                TsEntityName::Ident(identifier) => {
                    let type_name = identifier.sym.to_string();
                    symbol_tables.insert(
                        file_path,
                        name.to_string(),
                        Declaration::Alias {
                            from: name.to_string(),
                            to: type_name,
                        },
                    )
                }
                _ => {}
            },
            _ => store_variable(name, child, file_path, symbol_tables),
        }
    }
}
