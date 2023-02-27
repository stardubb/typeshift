#![allow(dead_code)]

pub const UNKNOWN: u16 = 0;
pub const END_OF_FILE_TOKEN: u16 = 1;
pub const SINGLE_LINE_COMMENT_TRIVIA: u16 = 2;
pub const MULTI_LINE_COMMENT_TRIVIA: u16 = 3;
pub const NEW_LINE_TRIVIA: u16 = 4;
pub const WHITESPACE_TRIVIA: u16 = 5;
pub const SHEBANG_TRIVIA: u16 = 6;
pub const CONFLICT_MARKER_TRIVIA: u16 = 7;
pub const NUMERIC_LITERAL: u16 = 8;
pub const BIG_INT_LITERAL: u16 = 9;
pub const STRING_LITERAL: u16 = 10;
pub const JSX_TEXT: u16 = 11;
pub const JSX_TEXT_ALL_WHITE_SPACES: u16 = 12;
pub const REGULAR_EXPRESSION_LITERAL: u16 = 13;
pub const NO_SUBSTITUTION_TEMPLATE_LITERAL: u16 = 14;
pub const TEMPLATE_HEAD: u16 = 15;
pub const TEMPLATE_MIDDLE: u16 = 16;
pub const TEMPLATE_TAIL: u16 = 17;
pub const OPEN_BRACE_TOKEN: u16 = 18;
pub const CLOSE_BRACE_TOKEN: u16 = 19;
pub const OPEN_PAREN_TOKEN: u16 = 20;
pub const CLOSE_PAREN_TOKEN: u16 = 21;
pub const OPEN_BRACKET_TOKEN: u16 = 22;
pub const CLOSE_BRACKET_TOKEN: u16 = 23;
pub const DOT_TOKEN: u16 = 24;
pub const DOT_DOT_DOT_TOKEN: u16 = 25;
pub const SEMICOLON_TOKEN: u16 = 26;
pub const COMMA_TOKEN: u16 = 27;
pub const QUESTION_DOT_TOKEN: u16 = 28;
pub const LESS_THAN_TOKEN: u16 = 29;
pub const LESS_THAN_SLASH_TOKEN: u16 = 30;
pub const GREATER_THAN_TOKEN: u16 = 31;
pub const LESS_THAN_EQUALS_TOKEN: u16 = 32;
pub const GREATER_THAN_EQUALS_TOKEN: u16 = 33;
pub const EQUALS_EQUALS_TOKEN: u16 = 34;
pub const EXCLAMATION_EQUALS_TOKEN: u16 = 35;
pub const EQUALS_EQUALS_EQUALS_TOKEN: u16 = 36;
pub const EXCLAMATION_EQUALS_EQUALS_TOKEN: u16 = 37;
pub const EQUALS_GREATER_THAN_TOKEN: u16 = 38;
pub const PLUS_TOKEN: u16 = 39;
pub const MINUS_TOKEN: u16 = 40;
pub const ASTERISK_TOKEN: u16 = 41;
pub const ASTERISK_ASTERISK_TOKEN: u16 = 42;
pub const SLASH_TOKEN: u16 = 43;
pub const PERCENT_TOKEN: u16 = 44;
pub const PLUS_PLUS_TOKEN: u16 = 45;
pub const MINUS_MINUS_TOKEN: u16 = 46;
pub const LESS_THAN_LESS_THAN_TOKEN: u16 = 47;
pub const GREATER_THAN_GREATER_THAN_TOKEN: u16 = 48;
pub const GREATER_THAN_GREATER_THAN_GREATER_THAN_TOKEN: u16 = 49;
pub const AMPERSAND_TOKEN: u16 = 50;
pub const BAR_TOKEN: u16 = 51;
pub const CARET_TOKEN: u16 = 52;
pub const EXCLAMATION_TOKEN: u16 = 53;
pub const TILDE_TOKEN: u16 = 54;
pub const AMPERSAND_AMPERSAND_TOKEN: u16 = 55;
pub const BAR_BAR_TOKEN: u16 = 56;
pub const QUESTION_TOKEN: u16 = 57;
pub const COLON_TOKEN: u16 = 58;
pub const AT_TOKEN: u16 = 59;
pub const QUESTION_QUESTION_TOKEN: u16 = 60;
pub const BACKTICK_TOKEN: u16 = 61;
pub const HASH_TOKEN: u16 = 62;
pub const EQUALS_TOKEN: u16 = 63;
pub const PLUS_EQUALS_TOKEN: u16 = 64;
pub const MINUS_EQUALS_TOKEN: u16 = 65;
pub const ASTERISK_EQUALS_TOKEN: u16 = 66;
pub const ASTERISK_ASTERISK_EQUALS_TOKEN: u16 = 67;
pub const SLASH_EQUALS_TOKEN: u16 = 68;
pub const PERCENT_EQUALS_TOKEN: u16 = 69;
pub const LESS_THAN_LESS_THAN_EQUALS_TOKEN: u16 = 70;
pub const GREATER_THAN_GREATER_THAN_EQUALS_TOKEN: u16 = 71;
pub const GREATER_THAN_GREATER_THAN_GREATER_THAN_EQUALS_TOKEN: u16 = 72;
pub const AMPERSAND_EQUALS_TOKEN: u16 = 73;
pub const BAR_EQUALS_TOKEN: u16 = 74;
pub const BAR_BAR_EQUALS_TOKEN: u16 = 75;
pub const AMPERSAND_AMPERSAND_EQUALS_TOKEN: u16 = 76;
pub const QUESTION_QUESTION_EQUALS_TOKEN: u16 = 77;
pub const CARET_EQUALS_TOKEN: u16 = 78;
pub const IDENTIFIER: u16 = 79;
pub const PRIVATE_IDENTIFIER: u16 = 80;
pub const BREAK_KEYWORD: u16 = 81;
pub const CASE_KEYWORD: u16 = 82;
pub const CATCH_KEYWORD: u16 = 83;
pub const CLASS_KEYWORD: u16 = 84;
pub const CONST_KEYWORD: u16 = 85;
pub const CONTINUE_KEYWORD: u16 = 86;
pub const DEBUGGER_KEYWORD: u16 = 87;
pub const DEFAULT_KEYWORD: u16 = 88;
pub const DELETE_KEYWORD: u16 = 89;
pub const DO_KEYWORD: u16 = 90;
pub const ELSE_KEYWORD: u16 = 91;
pub const ENUM_KEYWORD: u16 = 92;
pub const EXPORT_KEYWORD: u16 = 93;
pub const EXTENDS_KEYWORD: u16 = 94;
pub const FALSE_KEYWORD: u16 = 95;
pub const FINALLY_KEYWORD: u16 = 96;
pub const FOR_KEYWORD: u16 = 97;
pub const FUNCTION_KEYWORD: u16 = 98;
pub const IF_KEYWORD: u16 = 99;
pub const IMPORT_KEYWORD: u16 = 100;
pub const IN_KEYWORD: u16 = 101;
pub const INSTANCE_OF_KEYWORD: u16 = 102;
pub const NEW_KEYWORD: u16 = 103;
pub const NULL_KEYWORD: u16 = 104;
pub const RETURN_KEYWORD: u16 = 105;
pub const SUPER_KEYWORD: u16 = 106;
pub const SWITCH_KEYWORD: u16 = 107;
pub const THIS_KEYWORD: u16 = 108;
pub const THROW_KEYWORD: u16 = 109;
pub const TRUE_KEYWORD: u16 = 110;
pub const TRY_KEYWORD: u16 = 111;
pub const TYPE_OF_KEYWORD: u16 = 112;
pub const VAR_KEYWORD: u16 = 113;
pub const VOID_KEYWORD: u16 = 114;
pub const WHILE_KEYWORD: u16 = 115;
pub const WITH_KEYWORD: u16 = 116;
pub const IMPLEMENTS_KEYWORD: u16 = 117;
pub const INTERFACE_KEYWORD: u16 = 118;
pub const LET_KEYWORD: u16 = 119;
pub const PACKAGE_KEYWORD: u16 = 120;
pub const PRIVATE_KEYWORD: u16 = 121;
pub const PROTECTED_KEYWORD: u16 = 122;
pub const PUBLIC_KEYWORD: u16 = 123;
pub const STATIC_KEYWORD: u16 = 124;
pub const YIELD_KEYWORD: u16 = 125;
pub const ABSTRACT_KEYWORD: u16 = 126;
pub const ACCESSOR_KEYWORD: u16 = 127;
pub const AS_KEYWORD: u16 = 128;
pub const ASSERTS_KEYWORD: u16 = 129;
pub const ASSERT_KEYWORD: u16 = 130;
pub const ANY_KEYWORD: u16 = 131;
pub const ASYNC_KEYWORD: u16 = 132;
pub const AWAIT_KEYWORD: u16 = 133;
pub const BOOLEAN_KEYWORD: u16 = 134;
pub const CONSTRUCTOR_KEYWORD: u16 = 135;
pub const DECLARE_KEYWORD: u16 = 136;
pub const GET_KEYWORD: u16 = 137;
pub const INFER_KEYWORD: u16 = 138;
pub const INTRINSIC_KEYWORD: u16 = 139;
pub const IS_KEYWORD: u16 = 140;
pub const KEY_OF_KEYWORD: u16 = 141;
pub const MODULE_KEYWORD: u16 = 142;
pub const NAMESPACE_KEYWORD: u16 = 143;
pub const NEVER_KEYWORD: u16 = 144;
pub const OUT_KEYWORD: u16 = 145;
pub const READONLY_KEYWORD: u16 = 146;
pub const REQUIRE_KEYWORD: u16 = 147;
pub const NUMBER_KEYWORD: u16 = 148;
pub const OBJECT_KEYWORD: u16 = 149;
pub const SATISFIES_KEYWORD: u16 = 150;
pub const SET_KEYWORD: u16 = 151;
pub const STRING_KEYWORD: u16 = 152;
pub const SYMBOL_KEYWORD: u16 = 153;
pub const TYPE_KEYWORD: u16 = 154;
pub const UNDEFINED_KEYWORD: u16 = 155;
pub const UNIQUE_KEYWORD: u16 = 156;
pub const UNKNOWN_KEYWORD: u16 = 157;
pub const FROM_KEYWORD: u16 = 158;
pub const GLOBAL_KEYWORD: u16 = 159;
pub const BIG_INT_KEYWORD: u16 = 160;
pub const OVERRIDE_KEYWORD: u16 = 161;
pub const OF_KEYWORD: u16 = 162;
pub const QUALIFIED_NAME: u16 = 163;
pub const COMPUTED_PROPERTY_NAME: u16 = 164;
pub const TYPE_PARAMETER: u16 = 165;
pub const PARAMETER: u16 = 166;
pub const DECORATOR: u16 = 167;
pub const PROPERTY_SIGNATURE: u16 = 168;
pub const PROPERTY_DECLARATION: u16 = 169;
pub const METHOD_SIGNATURE: u16 = 170;
pub const METHOD_DECLARATION: u16 = 171;
pub const CLASS_STATIC_BLOCK_DECLARATION: u16 = 172;
pub const CONSTRUCTOR: u16 = 173;
pub const GET_ACCESSOR: u16 = 174;
pub const SET_ACCESSOR: u16 = 175;
pub const CALL_SIGNATURE: u16 = 176;
pub const CONSTRUCT_SIGNATURE: u16 = 177;
pub const INDEX_SIGNATURE: u16 = 178;
pub const TYPE_PREDICATE: u16 = 179;
pub const TYPE_REFERENCE: u16 = 180;
pub const FUNCTION_TYPE: u16 = 181;
pub const CONSTRUCTOR_TYPE: u16 = 182;
pub const TYPE_QUERY: u16 = 183;
pub const TYPE_LITERAL: u16 = 184;
pub const ARRAY_TYPE: u16 = 185;
pub const TUPLE_TYPE: u16 = 186;
pub const OPTIONAL_TYPE: u16 = 187;
pub const REST_TYPE: u16 = 188;
pub const UNION_TYPE: u16 = 189;
pub const INTERSECTION_TYPE: u16 = 190;
pub const CONDITIONAL_TYPE: u16 = 191;
pub const INFER_TYPE: u16 = 192;
pub const PARENTHESIZED_TYPE: u16 = 193;
pub const THIS_TYPE: u16 = 194;
pub const TYPE_OPERATOR: u16 = 195;
pub const INDEXED_ACCESS_TYPE: u16 = 196;
pub const MAPPED_TYPE: u16 = 197;
pub const LITERAL_TYPE: u16 = 198;
pub const NAMED_TUPLE_MEMBER: u16 = 199;
pub const TEMPLATE_LITERAL_TYPE: u16 = 200;
pub const TEMPLATE_LITERAL_TYPE_SPAN: u16 = 201;
pub const IMPORT_TYPE: u16 = 202;
pub const OBJECT_BINDING_PATTERN: u16 = 203;
pub const ARRAY_BINDING_PATTERN: u16 = 204;
pub const BINDING_ELEMENT: u16 = 205;
pub const ARRAY_LITERAL_EXPRESSION: u16 = 206;
pub const OBJECT_LITERAL_EXPRESSION: u16 = 207;
pub const PROPERTY_ACCESS_EXPRESSION: u16 = 208;
pub const ELEMENT_ACCESS_EXPRESSION: u16 = 209;
pub const CALL_EXPRESSION: u16 = 210;
pub const NEW_EXPRESSION: u16 = 211;
pub const TAGGED_TEMPLATE_EXPRESSION: u16 = 212;
pub const TYPE_ASSERTION_EXPRESSION: u16 = 213;
pub const PARENTHESIZED_EXPRESSION: u16 = 214;
pub const FUNCTION_EXPRESSION: u16 = 215;
pub const ARROW_FUNCTION: u16 = 216;
pub const DELETE_EXPRESSION: u16 = 217;
pub const TYPE_OF_EXPRESSION: u16 = 218;
pub const VOID_EXPRESSION: u16 = 219;
pub const AWAIT_EXPRESSION: u16 = 220;
pub const PREFIX_UNARY_EXPRESSION: u16 = 221;
pub const POSTFIX_UNARY_EXPRESSION: u16 = 222;
pub const BINARY_EXPRESSION: u16 = 223;
pub const CONDITIONAL_EXPRESSION: u16 = 224;
pub const TEMPLATE_EXPRESSION: u16 = 225;
pub const YIELD_EXPRESSION: u16 = 226;
pub const SPREAD_ELEMENT: u16 = 227;
pub const CLASS_EXPRESSION: u16 = 228;
pub const OMITTED_EXPRESSION: u16 = 229;
pub const EXPRESSION_WITH_TYPE_ARGUMENTS: u16 = 230;
pub const AS_EXPRESSION: u16 = 231;
pub const NON_NULL_EXPRESSION: u16 = 232;
pub const META_PROPERTY: u16 = 233;
pub const SYNTHETIC_EXPRESSION: u16 = 234;
pub const SATISFIES_EXPRESSION: u16 = 235;
pub const TEMPLATE_SPAN: u16 = 236;
pub const SEMICOLON_CLASS_ELEMENT: u16 = 237;
pub const BLOCK: u16 = 238;
pub const EMPTY_STATEMENT: u16 = 239;
pub const VARIABLE_STATEMENT: u16 = 240;
pub const EXPRESSION_STATEMENT: u16 = 241;
pub const IF_STATEMENT: u16 = 242;
pub const DO_STATEMENT: u16 = 243;
pub const WHILE_STATEMENT: u16 = 244;
pub const FOR_STATEMENT: u16 = 245;
pub const FOR_IN_STATEMENT: u16 = 246;
pub const FOR_OF_STATEMENT: u16 = 247;
pub const CONTINUE_STATEMENT: u16 = 248;
pub const BREAK_STATEMENT: u16 = 249;
pub const RETURN_STATEMENT: u16 = 250;
pub const WITH_STATEMENT: u16 = 251;
pub const SWITCH_STATEMENT: u16 = 252;
pub const LABELED_STATEMENT: u16 = 253;
pub const THROW_STATEMENT: u16 = 254;
pub const TRY_STATEMENT: u16 = 255;
pub const DEBUGGER_STATEMENT: u16 = 256;
pub const VARIABLE_DECLARATION: u16 = 257;
pub const VARIABLE_DECLARATION_LIST: u16 = 258;
pub const FUNCTION_DECLARATION: u16 = 259;
pub const CLASS_DECLARATION: u16 = 260;
pub const INTERFACE_DECLARATION: u16 = 261;
pub const TYPE_ALIAS_DECLARATION: u16 = 262;
pub const ENUM_DECLARATION: u16 = 263;
pub const MODULE_DECLARATION: u16 = 264;
pub const MODULE_BLOCK: u16 = 265;
pub const CASE_BLOCK: u16 = 266;
pub const NAMESPACE_EXPORT_DECLARATION: u16 = 267;
pub const IMPORT_EQUALS_DECLARATION: u16 = 268;
pub const IMPORT_DECLARATION: u16 = 269;
pub const IMPORT_CLAUSE: u16 = 270;
pub const NAMESPACE_IMPORT: u16 = 271;
pub const NAMED_IMPORTS: u16 = 272;
pub const IMPORT_SPECIFIER: u16 = 273;
pub const EXPORT_ASSIGNMENT: u16 = 274;
pub const EXPORT_DECLARATION: u16 = 275;
pub const NAMED_EXPORTS: u16 = 276;
pub const NAMESPACE_EXPORT: u16 = 277;
pub const EXPORT_SPECIFIER: u16 = 278;
pub const MISSING_DECLARATION: u16 = 279;
pub const EXTERNAL_MODULE_REFERENCE: u16 = 280;
pub const JSX_ELEMENT: u16 = 281;
pub const JSX_SELF_CLOSING_ELEMENT: u16 = 282;
pub const JSX_OPENING_ELEMENT: u16 = 283;
pub const JSX_CLOSING_ELEMENT: u16 = 284;
pub const JSX_FRAGMENT: u16 = 285;
pub const JSX_OPENING_FRAGMENT: u16 = 286;
pub const JSX_CLOSING_FRAGMENT: u16 = 287;
pub const JSX_ATTRIBUTE: u16 = 288;
pub const JSX_ATTRIBUTES: u16 = 289;
pub const JSX_SPREAD_ATTRIBUTE: u16 = 290;
pub const JSX_EXPRESSION: u16 = 291;
pub const CASE_CLAUSE: u16 = 292;
pub const DEFAULT_CLAUSE: u16 = 293;
pub const HERITAGE_CLAUSE: u16 = 294;
pub const CATCH_CLAUSE: u16 = 295;
pub const ASSERT_CLAUSE: u16 = 296;
pub const ASSERT_ENTRY: u16 = 297;
pub const IMPORT_TYPE_ASSERTION_CONTAINER: u16 = 298;
pub const PROPERTY_ASSIGNMENT: u16 = 299;
pub const SHORTHAND_PROPERTY_ASSIGNMENT: u16 = 300;
pub const SPREAD_ASSIGNMENT: u16 = 301;
pub const ENUM_MEMBER: u16 = 302;
pub const UNPARSED_PROLOGUE: u16 = 303;
pub const UNPARSED_PREPEND: u16 = 304;
pub const UNPARSED_TEXT: u16 = 305;
pub const UNPARSED_INTERNAL_TEXT: u16 = 306;
pub const UNPARSED_SYNTHETIC_REFERENCE: u16 = 307;
pub const SOURCE_FILE: u16 = 308;
pub const BUNDLE: u16 = 309;
pub const UNPARSED_SOURCE: u16 = 310;
pub const INPUT_FILES: u16 = 311;
pub const JS_DOC_TYPE_EXPRESSION: u16 = 312;
pub const JS_DOC_NAME_REFERENCE: u16 = 313;
pub const JS_DOC_MEMBER_NAME: u16 = 314;
pub const JS_DOC_ALL_TYPE: u16 = 315;
pub const JS_DOC_UNKNOWN_TYPE: u16 = 316;
pub const JS_DOC_NULLABLE_TYPE: u16 = 317;
pub const JS_DOC_NON_NULLABLE_TYPE: u16 = 318;
pub const JS_DOC_OPTIONAL_TYPE: u16 = 319;
pub const JS_DOC_FUNCTION_TYPE: u16 = 320;
pub const JS_DOC_VARIADIC_TYPE: u16 = 321;
pub const JS_DOC_NAMEPATH_TYPE: u16 = 322;
pub const JS_DOC: u16 = 323;
pub const JS_DOC_TEXT: u16 = 324;
pub const JS_DOC_TYPE_LITERAL: u16 = 325;
pub const JS_DOC_SIGNATURE: u16 = 326;
pub const JS_DOC_LINK: u16 = 327;
pub const JS_DOC_LINK_CODE: u16 = 328;
pub const JS_DOC_LINK_PLAIN: u16 = 329;
pub const JS_DOC_TAG: u16 = 330;
pub const JS_DOC_AUGMENTS_TAG: u16 = 331;
pub const JS_DOC_IMPLEMENTS_TAG: u16 = 332;
pub const JS_DOC_AUTHOR_TAG: u16 = 333;
pub const JS_DOC_DEPRECATED_TAG: u16 = 334;
pub const JS_DOC_CLASS_TAG: u16 = 335;
pub const JS_DOC_PUBLIC_TAG: u16 = 336;
pub const JS_DOC_PRIVATE_TAG: u16 = 337;
pub const JS_DOC_PROTECTED_TAG: u16 = 338;
pub const JS_DOC_READONLY_TAG: u16 = 339;
pub const JS_DOC_OVERRIDE_TAG: u16 = 340;
pub const JS_DOC_CALLBACK_TAG: u16 = 341;
pub const JS_DOC_ENUM_TAG: u16 = 342;
pub const JS_DOC_PARAMETER_TAG: u16 = 343;
pub const JS_DOC_RETURN_TAG: u16 = 344;
pub const JS_DOC_THIS_TAG: u16 = 345;
pub const JS_DOC_TYPE_TAG: u16 = 346;
pub const JS_DOC_TEMPLATE_TAG: u16 = 347;
pub const JS_DOC_TYPEDEF_TAG: u16 = 348;
pub const JS_DOC_SEE_TAG: u16 = 349;
pub const JS_DOC_PROPERTY_TAG: u16 = 350;
pub const SYNTAX_LIST: u16 = 351;
pub const NOT_EMITTED_STATEMENT: u16 = 352;
pub const PARTIALLY_EMITTED_EXPRESSION: u16 = 353;
pub const COMMA_LIST_EXPRESSION: u16 = 354;
pub const MERGE_DECLARATION_MARKER: u16 = 355;
pub const END_OF_DECLARATION_MARKER: u16 = 356;
pub const SYNTHETIC_REFERENCE_EXPRESSION: u16 = 357;
pub const COUNT: u16 = 358;