#![allow(bad_style, missing_docs, unreachable_pub)]
use codespan_reporting::diagnostic::Diagnostic;
use num_derive::{
    FromPrimitive,
    ToPrimitive,
};
use strum_macros::EnumCount;

#[cfg(test)]
mod smoke_test_tree_traversal {
    use pretty_assertions_sorted::assert_eq;

    use super::*;

    #[test]
    fn test_find_child() {
        let tree = Tree {
            kind:     TreeKind::TranslationUnit,
            range:    Span::new(0, 10),
            children: vec![
                Child::Tree(Tree {
                    kind:     TreeKind::FunctionDef,
                    range:    Span::new(1, 5),
                    children: vec![],
                    file_id:  0,
                }),
                Child::Tree(Tree {
                    kind:     TreeKind::ExpressionStatement,
                    range:    Span::new(6, 9),
                    children: vec![],
                    file_id:  0,
                }),
            ],
            file_id:  0,
        };

        // Test finding a specific child by kind.
        assert!(tree.find_child(TreeKind::FunctionDef).is_some());
        // assert_eq!(tree.find_child(TreeKind::FunctionDef).unwrap().range, Span {
        //     start: 1,
        //     end:   5,
        // });

        // Test finding a child that doesn't exist.
        assert!(tree.find_child(TreeKind::SpecifierQualifierList).is_none());
    }

    #[test]
    fn test_find_token() {
        let token1 = Token::new(TokenKind::IDENTIFIER, "foo".into(), Span::new(0, 3));
        let token2 = Token::new(TokenKind::INTEGER_CONSTANT, "42".into(), Span::new(4, 5));
        let token3 = Token::new(TokenKind::STRING, "\"hello\"".into(), Span::new(6, 12));

        let tree = Tree {
            kind:     TreeKind::Expression,
            range:    Span::new(0, 12),
            children: vec![
                Child::Token(token1),
                Child::Token(token2.clone()),
                Child::Token(token3),
            ],
            file_id:  0,
        };

        // Test finding a specific token by kind.
        assert_eq!(tree.find_token(TokenKind::INTEGER_CONSTANT), Some(&token2));

        // Test finding a token that doesn't exist.
        assert_eq!(tree.find_token(TokenKind::FLOATING_CONSTANT), None);
    }
}

#[allow(clippy::manual_non_exhaustive, non_snake_case, non_camel_case_types)]
#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, FromPrimitive, ToPrimitive, EnumCount,
)]
#[repr(u16)]
pub enum SyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc(hidden)]
    EOF,
    ///Literals (e.g. IDENTIFIER, INT, FLOAT, STRING, BYTES)
    IDENTIFIER,
    INT,
    FLOAT,
    STRING,
    BYTES,
    ///Tokens (e.g. WHITESPACE, COMMENT, NEWLINE)
    WHITESPACE,
    COMMENT,
    NEWLINE,
    INDENT,
    OUTDENT,
    UNKNOWN,
    CONSTANT,
    ///Keywords (e.g. BREAK, IN, LET, LOOP, etc.)
    AND_KW,
    ELLIPSIS,
    AUTO_KW,
    CASE_KW,
    CHAR_KW,
    CONST_KW,
    DEFAULT_KW,
    DO_KW,
    DOUBLE_KW,
    ENUM_KW,
    EXTERN_KW,
    ELSE_KW,
    FLOAT_KW,
    LOAD_KW,
    BREAK_KW,
    FOR_KW,
    NOT_KW,
    GOTO_KW,
    RUNE,
    CONTINUE_KW,
    IF_KW,
    INT_KW,
    LONG_KW,
    OR_KW,
    DEF_KW,
    IN_KW,
    PASS_KW,
    ELIF_KW,
    REGISTER_KW,
    LAMBDA_KW,
    SHORT_KW,
    SIGNED_KW,
    SIZEOF_KW,
    STATIC_KW,
    STRUCT_KW,
    SWITCH_KW,
    TYPEDEF_KW,
    UNION_KW,
    UNSIGNED_KW,
    VOID_KW,
    RETURN_KW,
    VOLATILE_KW,
    WHILE_KW,
    ///Punctuation (e.g. DOT, COMMA, SEMICOLON, etc.)
    PLUS,
    MINUS,
    STAR,
    SLASH,
    DSLASH,
    PERCENT,
    DSTAR,
    TILDE,
    AMP,
    DOUBLEAMP,
    PIPE,
    BANG,
    DOUBLEPIPE,
    CARET,
    LSHIFT,
    RSHIFT,
    DOT,
    COMMA,
    EQ,
    SEMICOLON,
    COLON,
    LPAREN,
    RPAREN,
    LBRACKET,
    RBRACKET,
    LBRACE,
    RBRACE,
    LT,
    GT,
    GE,
    LE,
    EQEQ,
    NE,
    PLUSEQ,
    MINUSEQ,
    STAREQ,
    SLASHEQ,
    PERCENTEQ,
    AMPEQ,
    PIPEEQ,
    CARETEQ,
    LSHIFTEQ,
    RSHIFTEQ,
    DEC_OP,
    INC_OP,
    PTR_OP,
    QUESTION,
    DSLASHEQ,
    ///Nodes (e.g. FILE, MODULE, FUNCTION, etc.)
    FILE,
    STATEMENT,
    DEF_STMT,
    PARAMETERS,
    PARAMETER,
    IF_STMT,
    ELIF_CLAUSES,
    ELSE_CLAUSE,
    FOR_STMT,
    SUITE,
    SIMPLE_STMT,
    SMALL_STMT,
    RETURN_STMT,
    BREAK_STMT,
    CONTINUE_STMT,
    PASS_STMT,
    ASSIGN_STMT,
    EXPR_STMT,
    LOAD_STMT,
    TEST,
    BIN_OP,
    IF_EXPR,
    PRIMARY_EXPR,
    OPERAND,
    DOT_SUFFIX,
    SLICE_SUFFIX,
    CALL_SUFFIX,
    ARGUMENTS,
    ARGUMENT,
    LIST_EXPR,
    LIST_COMP,
    DICT_EXPR,
    DICT_COMP,
    ENTRIES,
    ENTRY,
    COMP_CLAUSE,
    UNARY_EXPR,
    BINARY_EXPR,
    BINOP,
    LAMBDA_EXPR,
    EXPRESSION,
    LOOP_VARIABLES,
    #[doc(hidden)]
    __LAST,
    INLINE_KW,
    RESTRICT_KW,
    ALIGNAS_KW,
    ALIGNOF_KW,
    ATOMIC_KW,
    BOOL_KW,
    COMPLEX_KW,
    GENERIC_KW,
    IMAGINARY_KW,
    NORETURN_KW,
    STATIC_ASSERT_KW,
    THREAD_LOCAL_KW,
    INTEGER_CONSTANT,
    FLOATING_CONSTANT,
    FUNC_NAME_KW,
}
use self::SyntaxKind::*;
impl SyntaxKind {
    pub fn is_keyword(self) -> bool {
        matches!(
            self,
            AND_KW |
                ELSE_KW |
                LOAD_KW |
                BREAK_KW |
                FOR_KW |
                NOT_KW |
                CONTINUE_KW |
                IF_KW |
                OR_KW |
                DEF_KW |
                IN_KW |
                PASS_KW |
                ELIF_KW |
                LAMBDA_KW |
                RETURN_KW
        )
    }
    pub fn is_punct(self) -> bool {
        matches!(
            self,
            PLUS | MINUS |
                STAR |
                SLASH |
                DSLASH |
                PERCENT |
                DSTAR |
                TILDE |
                AMP |
                PIPE |
                CARET |
                LSHIFT |
                RSHIFT |
                DOT |
                COMMA |
                EQ |
                SEMICOLON |
                COLON |
                LPAREN |
                RPAREN |
                LBRACKET |
                RBRACKET |
                LBRACE |
                RBRACE |
                LT |
                GT |
                GE |
                LE |
                EQEQ |
                NE |
                PLUSEQ |
                MINUSEQ |
                STAREQ |
                SLASHEQ |
                PERCENTEQ |
                AMPEQ |
                PIPEEQ |
                CARETEQ |
                LSHIFTEQ |
                RSHIFTEQ
        )
    }
    pub fn is_literal(self) -> bool {
        matches!(self, IDENTIFIER | INT | FLOAT | STRING | BYTES)
    }
    pub fn from_keyword(ident: &str) -> Option<SyntaxKind> {
        let kw = match ident {
            "and" => AND_KW,
            "else" => ELSE_KW,
            "load" => LOAD_KW,
            "break" => BREAK_KW,
            "for" => FOR_KW,
            "not" => NOT_KW,
            "continue" => CONTINUE_KW,
            "if" => IF_KW,
            "or" => OR_KW,
            "def" => DEF_KW,
            "in" => IN_KW,
            "pass" => PASS_KW,
            "elif" => ELIF_KW,
            "lambda" => LAMBDA_KW,
            "return" => RETURN_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub fn from_contextual_keyword(ident: &str) -> Option<SyntaxKind> {
        let kw = match ident {
            _ => return None,
        };
        Some(kw)
    }
    pub fn from_char(c: char) -> Option<SyntaxKind> {
        let tok = match c {
            '+' => PLUS,
            '-' => MINUS,
            '*' => STAR,
            '/' => SLASH,
            '%' => PERCENT,
            '~' => TILDE,
            '&' => AMP,
            '|' => PIPE,
            '^' => CARET,
            '.' => DOT,
            ',' => COMMA,
            '=' => EQ,
            ';' => SEMICOLON,
            ':' => COLON,
            '(' => LPAREN,
            ')' => RPAREN,
            '[' => LBRACKET,
            ']' => RBRACKET,
            '{' => LBRACE,
            '}' => RBRACE,
            '<' => LT,
            '>' => GT,
            _ => return None,
        };
        Some(tok)
    }
}

use crate::{
    ast::{
        AstSink,
        Block,
        DataType,
        Declaration,
        Expr,
        ExternDecl,
        Function,
        Literal,
        Param,
        Statement,
        Symbol,
        TranslationUnit,
    },
    diagnostics::{
        self,
        DiagnosticsEngine,
        FileId,
    },
    lexer::{
        Span,
        Token,
        TokenKind,
    },
};
use owo_colors::OwoColorize;
// use smartstring::alias::String;
use std::{
    default,
    fmt::{
        self,
        Display,
    },
};
use strum_macros::Display;

#[macro_export]
macro_rules! format_to {
    ($buf:expr) => ();
    ($buf:expr, $lit:literal $($arg:tt)*) => {
        { use ::std::fmt::Write as _; let _ = ::std::write!($buf, $lit $($arg)*); }
    };
}

#[derive(Debug, Default, Display, PartialEq, Eq, Clone, Copy)]
pub enum TreeKind {
    #[default]
    Unknown,
    TranslationUnit,
    StaticAssertDeclaration,
    AtomicTypeSpecifier,
    ExpressionStatement,
    IterationStatement,
    InitializerList,
    JumpStatement,
    UnaryOperator,
    ParamList,
    ParameterDeclaration,
    Initializer,
    StructDeclarator,
    EnumSpecifier,
    Enumerator,
    Expression,
    ConditionalExpression,
    EnumeratorList,
    StructOrUnionSpecifier,
    PrimaryExpression,
    StructDeclaratorList,
    StructDeclaration,
    StructDeclarationList,
    ArgumentExpressionList,
    TypeName,
    SpecifierQualifierList,
    DirectDeclarator,
    ErrorTree,
    CompoundStatement,
    LogicalAndExpression,
    ExternDecl,
    File,
    PostfixExpression,
    InclusiveOrExpression,
    ExclusiveOrExpression,
    AndExpression,
    EqualityExpression,
    RelationalExpression,
    ShiftExpression,
    AdditiveExpression,
    MultiplicativeExpression,
    CastExpression,
    UnaryExpression,
    IdentifierList,
    StatementList,
    DirectAbstractDeclarator,
    Fn,
    TypeExpr,
    LogicalOrExpression,
    Pointer,
    Declaration,
    DeclarationList,
    InitDeclaratorList,
    TypeQualifierList,
    InitDeclarator,
    Declarator,
    TypeSpecifier,
    TypeQualifier,
    Param,
    Block,
    StmtLet,
    StorageClassSpecifier,
    StmtReturn,
    StmtExpr,
    ExprLiteral,
    ExprName,
    ExprParen,
    ExprBinary,
    ExprCall,
    ArgList,
    Arg,
    DeclarationSpecifiers,
    FunctionDef,
    Statement,
    LabeledStatement,
    SelectionStatement,
    AssignmentExpression,
    ConstantExpression,
    FunctionSpecifier,
    AlignmentSpecifier,
    Constant,
    String,
    GenericSelection,
    GenericAssocList,
    GenericAssociation,
    ParamTypeList,
    StructOrUnion,
    AbstractDeclarator,
    BlockItemList,
    BlockItem,
    Designation,
    DesignatorList,
    Designator,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct TreeSink {
    pub(crate) tree:          Tree,
    pub(crate) syntax_errors: Vec<Diagnostic<FileId>>,
}

impl TreeSink {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn num_errors(&self) -> usize {
        self.syntax_errors.len()
    }

    pub fn finish(mut self) -> (Tree, Vec<Diagnostic<FileId>>) {
        self.tree.children.retain(|child| match child {
            Child::Tree(tree) => tree.kind != TreeKind::ErrorTree,
            _ => true,
        });
        (self.tree, self.syntax_errors)
    }

    pub fn start_node(&mut self, kind: TreeKind, range: Span) {
        let tree = Tree { kind, range, children: Vec::new(), file_id: 0 };
        self.tree.children.push(Child::Tree(tree));
    }

    pub fn finish_node(&mut self) {
        let tree = self.tree.children.pop().unwrap();
        match tree {
            Child::Tree(tree) => self.tree.children.push(Child::Tree(tree)),
            _ => unreachable!(),
        }
    }

    pub fn error(&mut self, error: Diagnostic<usize>) {
        self.syntax_errors.push(error);
    }

    pub fn token(&mut self, kind: TokenKind, lexeme: String, span: Span) {
        let token = Token::new(kind, lexeme, span);
        self.tree.children.push(Child::Token(token));
    }

    pub fn push_error(&mut self, error: Diagnostic<usize>) {
        self.syntax_errors.push(error);
    }

    pub fn drain_errors(&mut self, diagnostics: &mut DiagnosticsEngine) {
        for error in self.syntax_errors.drain(..) {
            diagnostics.emit(error.clone());
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Tree {
    pub(crate) kind:     TreeKind,
    pub(crate) range:    Span,
    pub(crate) children: Vec<Child>,
    pub(crate) file_id:  FileId,
}

#[derive(Debug, Display, PartialEq, Eq, Clone)]
pub enum Child {
    Token(Token),
    Tree(Tree),
}

impl Tree {
    // Helper function to find a child node of a specific kind.
    fn find_child(&self, kind: TreeKind) -> Option<&Tree> {
        self.children
            .iter()
            .find(|&child| match child {
                Child::Tree(tree) => tree.kind == kind,
                _ => false,
            })
            .and_then(|child| match child {
                Child::Tree(tree) => Some(tree),
                _ => None,
            })
    }

    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }

    // Helper function to find a token of a specific kind.
    fn find_token(&self, kind: TokenKind) -> Option<&Token> {
        self.children
            .iter()
            .filter_map(|child| match child {
                Child::Token(token) if token.kind() == &kind => Some(token),
                _ => None,
            })
            .next()
    }

    pub fn nth_child(&self, n: usize) -> Option<&Child> {
        self.children.get(n)
    }

    pub fn num_functions(&self) -> usize {
        self.children
            .iter()
            .filter(|child| {
                // println!("num_functions - child: {:#?}", child);
                match child {
                    Child::Tree(tree) => {
                        if let TreeKind::ExternDecl = tree.kind {
                            if let Some(_function_def) = tree.find_child(TreeKind::FunctionDef) {
                                return true;
                            }

                            if let Some(_declaration) = tree.find_child(TreeKind::Declaration) {
                                return false;
                            }

                            false
                        } else {
                            false
                        }
                    }
                    _ => false,
                }
            })
            .count()
    }

    pub fn num_declarations(&self) -> usize {
        self.children
            .iter()
            .filter(|child| match child {
                Child::Tree(tree) => {
                    if let TreeKind::ExternDecl = tree.kind {
                        if let Some(_function_def) = tree.find_child(TreeKind::FunctionDef) {
                            return false;
                        }

                        if let Some(_declaration) = tree.find_child(TreeKind::Declaration) {
                            return true;
                        }

                        false
                    } else {
                        false
                    }
                }

                _ => false,
            })
            .count()
    }

    // fn find_child(&self, kind: TreeKind) -> Option<&Tree> {
    // self.children
    //     .iter()
    //     .find(|&&child| child.kind == kind)
    //     .map(|&child| match child {
    //         Child::Tree(tree) => Some(tree),
    //         _ => None,
    //     })
    //     .flatten()
    //     .as_ref()
    // }

    pub fn print(&self, buf: &mut String, level: usize) {
        let indent = "  ".repeat(level);

        // Print the tree node with its range.
        if level == 0 {
            format_to!(
                buf,
                "{indent}{}{}{}\n",
                self.kind.green(),
                "@".yellow(),
                self.range.black().italic()
            );
        } else {
            format_to!(
                buf,
                "{indent}{}{}{}{}\n",
                "└─".black(),
                self.kind.green(),
                "@".yellow(),
                self.range.black().italic()
            );
        }

        for child in &self.children {
            match child {
                Child::Token(token) => {
                    // Print the token with its range.
                    format_to!(
                        buf,
                        "{indent}  {} {:?}@{} {}{}{}\n",
                        "\\-".magenta(),
                        token.kind().blue(),
                        token.span().to_string().black().italic(),
                        "'".red(),
                        token.lexeme,
                        "'".red(),
                    );
                }
                Child::Tree(tree) => tree.print(buf, level + 1),
            }
        }
        assert!(buf.ends_with('\n'));
    }

    // Matches certain patterns on parse trees and transforms them into
    // AST nodes. AST nodes are a more minimal representation of thee
    // parse tree that is easier to work with, and is more suitable for
    // analysis and code generation.
    pub fn lower(&mut self) -> AstSink {
        let mut functions = Vec::<ExternDecl>::new();

        // Get the child node of the ExternDecl
        // let child = &mut extern_decl.children[0];

        // if let Child::Tree(child) = child {
        //     if child.kind == TreeKind::FunctionDef {
        //         functions.push(ExternDecl::Function(child.
        // transform_function()));     }

        for child in &mut self.children {
            if let Child::Tree(extern_decl) = child {
                // Go to the child node of the ExternDecl
                if extern_decl.kind == TreeKind::ExternDecl {
                    // Find the child node of ExternDecl that is a FunctionDef.
                    if let Some(function_def) = extern_decl.find_child(TreeKind::FunctionDef) {
                        functions.push(ExternDecl::Function(function_def.transform_function()));
                    } else if let Some(declaration) = extern_decl.find_child(TreeKind::Declaration)
                    {
                        functions
                            .push(ExternDecl::Declaration(declaration.transform_declaration()));
                    } else {
                        unreachable!("Expected FunctionDef or Declaration node")
                    }
                } else {
                    unreachable!("Expected ExternDecl node")
                }
            }
        }

        AstSink { translation_unit: TranslationUnit { functions }, syntax_errors: Vec::new() }
    }

    pub fn lower_with_diagnostics(&mut self, diagnostics: &mut DiagnosticsEngine) -> AstSink {
        let mut functions = Vec::<ExternDecl>::new();

        for child in &mut self.children {
            if let Child::Tree(extern_decl) = child {
                // Go to the child node of the ExternDecl
                if extern_decl.kind == TreeKind::ExternDecl {
                    // Find the child node of ExternDecl that is a FunctionDef.
                    if let Some(function_def) = extern_decl.find_child(TreeKind::FunctionDef) {
                        functions.push(ExternDecl::Function(
                            function_def.clone().transform_function_with_diagnostics(diagnostics),
                        ));
                        // functions.push(ExternDecl::Function(function_def.
                        // transform_function()));
                    } else if let Some(declaration) = extern_decl.find_child(TreeKind::Declaration)
                    {
                        functions
                            .push(ExternDecl::Declaration(declaration.transform_declaration()));
                    } else {
                        unreachable!("Expected FunctionDef or Declaration node")
                    }
                } else {
                    unreachable!("Expected ExternDecl node")
                }
            }
        }

        AstSink { translation_unit: TranslationUnit { functions }, syntax_errors: Vec::new() }
    }

    fn push_error(&mut self, error: Diagnostic<usize>, diagnostics: &mut DiagnosticsEngine) {
        diagnostics.emit(error);
    }

    fn transform_function_with_diagnostics(
        &mut self,
        diagnostics: &mut DiagnosticsEngine,
    ) -> Function {
        let (return_type, params, name) = self.extract_function_signature();
        let body = self.extract_function_body();

        // if !body.has_statements() {
        //     tracing::warn!(
        //         "{}",
        //         &format!(
        //             "  {}  Function {}@{} has no statements",
        //             "PARSER".yellow(),
        //             name.green(),
        //             self.range.to_string().black().italic(),
        //         )
        //     );
        // }

        // If we have a return type, but no block, then emit an error.
        // testdata/parse/b.c:56:17: warning: non-void function does not return a value
        // [-Wreturn-type] int qux(int x) {}
        //                 ^
        // 1 warning generated.
        if return_type != Box::new(DataType::Void) && !body.has_statements() {
            tracing::warn!(
                "{}",
                &format!(
                    "  {}  Function {}@{} has no statements",
                    "PARSER".yellow(),
                    name.green(),
                    self.range.to_string().black().italic(),
                )
            );

            // non-void function does not return a value

            // self.ast_sink.
            // push_error(diagnostics::non_void_function_doesnt_return_value(
            //     self.file_id,
            //     self.range,
            // ));
            self.push_error(
                diagnostics::non_void_function_doesnt_return_value(self.file_id, self.range),
                diagnostics,
            );

            println!("diag {:#?}", diagnostics);

            // if let Some(mut ast_sink) = self.ast_sink.clone() {
            //     // println!("ast_sink {:#?}", ast_sink);
            //     ast_sink.
            // push_error(diagnostics::non_void_function_doesnt_return_value(
            //         self.file_id,
            //         self.range,
            //     ));
            // }

            // self.tree_sink.
            // push_error(diagnostics::unexpected_token_diagnostic(
            //         self.file_id,
            //         &unexpected_token,
            //         &expected,
            //     ));
        }

        tracing::trace!(
            "{}",
            &format!(
                "{} Lowering {}@{} to {} {} {}{}{}",
                "PARSER".yellow(),
                "FunctionDef".green(),
                self.range.to_string().black().italic(),
                "Function".cyan(),
                "-".red(),
                " ".yellow(),
                name.green(),
                " ".yellow(),
            )
        );

        Function { name: Symbol { name }, params, return_type, body }
    }

    fn transform_function(&self) -> Function {
        let (return_type, params, name) = self.extract_function_signature();
        let body = self.extract_function_body();

        // if !body.has_statements() {
        //     tracing::warn!(
        //         "{}",
        //         &format!(
        //             "  {}  Function {}@{} has no statements",
        //             "PARSER".yellow(),
        //             name.green(),
        //             self.range.to_string().black().italic(),
        //         )
        //     );
        // }

        // If we have a return type, but no block, then emit an error.
        // testdata/parse/b.c:56:17: warning: non-void function does not return a value
        // [-Wreturn-type] int qux(int x) {}
        //                 ^
        // 1 warning generated.
        if return_type != Box::new(DataType::Void) && !body.has_statements() {
            tracing::warn!(
                "{}",
                &format!(
                    "  {}  Function {}@{} has no statements",
                    "PARSER".yellow(),
                    name.green(),
                    self.range.to_string().black().italic(),
                )
            );
        }

        tracing::trace!(
            "{}",
            &format!(
                "{} Lowering {}@{} to {} {} {}{}{}",
                "PARSER".yellow(),
                "FunctionDef".green(),
                self.range.to_string().black().italic(),
                "Function".cyan(),
                "-".red(),
                " ".yellow(),
                name.green(),
                " ".yellow(),
            )
        );

        Function { name: Symbol { name }, params, return_type, body }
    }

    fn extract_parameter_info(&self, param: &Tree) -> (DataType, String) {
        let mut param_type = DataType::Int;
        let mut param_name = String::new();

        if param.kind == TreeKind::ParameterDeclaration {
            // println!("param {:#?}", param);
            if let Child::Tree(declaration_specifiers) = &param.children[0] {
                if let Child::Tree(type_specifier) = &declaration_specifiers.children[0] {
                    if type_specifier.kind == TreeKind::TypeSpecifier {
                        param_type = type_specifier.transform_type();
                    }
                }
            }

            // Start iterating through the children.
            let mut pointer_count = 0;
            let mut data_type_idx = 1;

            // println!("param.children.len() {:#?}", param.children.len());

            while data_type_idx < param.children.len() {
                // println!("param.children[{}] {:#?}", data_type_idx,
                // param.children[data_type_idx]);
                if let Child::Tree(param_child) = &param.children[data_type_idx] {
                    if param_child.kind == TreeKind::Pointer {
                        pointer_count += 1;

                        // Found a pointer, so go deeper into the tree to check for more pointers.
                        let mut pointer_child = param_child;
                        // println!("pointer {:#?}", pointer_child);

                        while pointer_child.children.len() > 1 {
                            if let Child::Tree(inner_pointer) = &pointer_child.children[1] {
                                // println!("inner_pointer {:#?}", inner_pointer);
                                if inner_pointer.kind == TreeKind::Pointer {
                                    pointer_count += 1;
                                    pointer_child = inner_pointer;
                                } else {
                                    // Handle other cases or report an error if needed.
                                    // TODO: Add error handling or logging.

                                    tracing::error!(
                                        "{}",
                                        &format!(
                                            "  {}  Unexpected node while lowering {}@{} to {}. \
                                             Expected {} or {} but found {}",
                                            "PARSER".yellow(),
                                            inner_pointer.kind.to_string().green(),
                                            inner_pointer.range.to_string().black().italic(),
                                            "ParameterDeclaration".cyan(),
                                            "Pointer".cyan(),
                                            "DirectDeclarator".cyan(),
                                            inner_pointer.kind.to_string().green(),
                                        )
                                    );
                                    break; // Exit the loop on error.
                                }
                            } else {
                                // Handle unexpected nodes or report an error if needed.
                                // TODO: Add error handling or logging.
                                break; // Exit the loop on error.
                            }
                        }
                    } else if param_child.kind == TreeKind::DirectDeclarator {
                        // println!("direct_declarator {param_child:#?}");

                        // We found the direct declarator, so extract the parameter name.
                        if let Child::Token(token) = &param_child.children[0] {
                            if token.kind == TokenKind::IDENTIFIER {
                                param_name = token.lexeme.clone();

                                tracing::trace!(
                                    "{}",
                                    &format!(
                                        "{} Lowering {}@{} to {} {} {}{}{}",
                                        "PARSER".yellow(),
                                        token.kind.to_string().blue(),
                                        token.span().to_string().black().italic(),
                                        "ParameterName".cyan(),
                                        "-".red(),
                                        " ".yellow().on_black(),
                                        token.lexeme.red().on_black(),
                                        " ".yellow().on_black(),
                                    )
                                );
                            } else {
                                // TODO: Error handling

                                tracing::error!(
                                    "{}",
                                    &format!(
                                        "  {}  Unexpected node while lowering {}@{} to {}. \
                                         Expected {} but found {}",
                                        "PARSER".yellow(),
                                        token.kind.to_string().green(),
                                        token.span.to_string().black().italic(),
                                        "Token".cyan(),
                                        "IDENTIFIER".cyan(),
                                        token.kind.to_string().green(),
                                    )
                                );
                            }
                        }
                        break; // Exit the loop since we've found the
                               // declarator.
                    } else {
                        // Handle other cases or report an error if needed.
                        // TODO: Add error handling or logging.

                        tracing::error!(
                            "{}",
                            &format!(
                                "  {}  Unexpected node while lowering {}@{} to {}. Expected {} or \
                                 {} but found {}",
                                "PARSER".yellow(),
                                param_child.kind.to_string().green(),
                                param_child.range.to_string().black().italic(),
                                "ParameterDeclaration".cyan(),
                                "Pointer".cyan(),
                                "DirectDeclarator".cyan(),
                                param_child.kind.to_string().green(),
                            )
                        );
                    }
                } else {
                    // Handle unexpected nodes or report an error if needed.
                    // TODO: Add error handling or logging.
                }

                data_type_idx += 1;
            }

            // Create the parameter data type based on pointer count.
            for _ in 0..pointer_count {
                param_type = DataType::Pointer(Box::new(param_type));
            }
        } else {
            // TODO: Error handling
            tracing::error!(
                "{}",
                &format!(
                    "  {}  Unexpected node while lowering {}@{} to {}. Expected {} but found {}",
                    "PARSER".yellow(),
                    param.kind.to_string().green(),
                    param.range.to_string().black().italic(),
                    "ParameterDeclaration".cyan(),
                    "ParameterDeclaration".cyan(),
                    param.kind.to_string().green(),
                )
            );
        }

        // TODO: update logging here (make param type range
        // accurate)
        tracing::trace!(
            "{}",
            &format!(
                "{} Lowering {}@{} to {} {} {}",
                "PARSER".yellow(),
                param.kind.to_string().green(),
                param.range.to_string().black().italic(),
                "DataType".cyan(),
                "-".red(),
                param_type.magenta()
            )
        );

        (param_type, param_name)
    }

    // Check if there are any pointer tokens in the parameter type
    // while let Child::Tree(pointer) = &param.children[0] {
    //     if pointer.kind == TreeKind::Pointer {
    //         param_type = DataType::Pointer(Box::new(param_type));
    //     }
    // }

    // if let Child::Tree(direct_declarator) = &param.children[1] {
    //     if let Child::Token(token) = &direct_declarator.children[0] {
    //         param_name = token.lexeme.clone();
    //     }
    // }

    // Check if there are any pointer tokens in the parameter type
    // println!("param children[0] {:#?}", param.children[0]);
    // for child in &param.children[0] {
    //     if let Child::Tree(pointer) = child {
    //         if pointer.kind == TreeKind::Pointer {
    //             param_type = DataType::Pointer(Box::new(param_type));
    //         }
    //     }
    // }

    // V1 START
    // // Check if there are any pointer tokens in the parameter type
    // let mut pointer_count = 0;

    // let mut data_type_idx = 1;
    // // println!("param.children[1] {:#?}", param.children[1]);
    // if let Child::Tree(param_child) = &param.children[data_type_idx] {
    //     if param_child.kind == TreeKind::Pointer {
    //         pointer_count += 1;
    //         data_type_idx += 1;
    //     }

    //     if let Child::Tree(param_child) = &param.children[data_type_idx] {
    //         if param_child.kind == TreeKind::Pointer {
    //             pointer_count += 1;
    //             data_type_idx += 1;
    //         }
    //     }

    //     if let Child::Tree(param_child) = &param.children[data_type_idx] {
    //         if param_child.kind == TreeKind::DirectDeclarator {
    //             if let Child::Token(token) = &param_child.children[0] {
    //                 param_name = token.lexeme.clone();
    //             }
    //         }
    //     }
    // } else {
    //     // TODO: Error handling
    //     tracing::error!(
    //         "{}",
    //         &format!(
    //             "  {}  Unexpected node while lowering {}@{} to {}. Expected {}
    // but found \              {}",
    //             "PARSER".yellow(),
    //             param.kind.to_string().green(),
    //             param.range.to_string().black().italic(),
    //             "ParameterDeclaration".cyan(),
    //             "ParameterDeclaration".cyan(),
    //             param.kind.to_string().green(),
    //         )
    //     );
    // }
    // for _ in 0..pointer_count {
    //     param_type = DataType::Pointer(Box::new(param_type));
    // }
    // V1 END

    fn extract_function_signature(&self) -> (Box<DataType>, Vec<Param>, String) {
        let mut name = String::new();
        let mut params = vec![];
        let mut return_type = Box::new(DataType::Int);

        let decl_specifiers = &self.children[0];

        if let Child::Tree(decl_specifiers) = decl_specifiers {
            if decl_specifiers.kind == TreeKind::DeclarationSpecifiers {
                if let Child::Tree(type_specifier) = &decl_specifiers.children[0] {
                    if type_specifier.kind == TreeKind::TypeSpecifier {
                        return_type = Box::new(type_specifier.transform_type());
                    }
                }
            } else {
                // TODO: Error handling
                tracing::error!(
                    "Expected DeclarationSpecifiers node while lowering function declaration"
                );
            }
        }

        let declarator = self.get_declarator();

        if let Child::Tree(direct_declarator) = &declarator.children[0] {
            if direct_declarator.kind == TreeKind::DirectDeclarator {
                // Get the function name
                if let Child::Token(token) = &direct_declarator.children[0] {
                    if token.kind == TokenKind::IDENTIFIER {
                        tracing::trace!(
                            "{}",
                            &format!(
                                "{} Lowering {}@{} to {} {} {}{}{}",
                                "PARSER".yellow(),
                                token.kind.to_string().blue(),
                                token.span().to_string().black().italic(),
                                "FunctionName".cyan(),
                                "-".red(),
                                " ".yellow().on_black(),
                                token.lexeme.green().on_black(),
                                " ".yellow().on_black(),
                            )
                        );
                        name = token.lexeme.clone();
                    } else {
                        // TODO: Error handling
                        tracing::error!(
                            "Expected Identifier token while lowering function declaration"
                        );
                    }
                }

                // Get the function parameters (if any)
                if let Child::Tree(parameter_type_list) = &direct_declarator.children[2] {
                    if parameter_type_list.kind == TreeKind::ParamTypeList {
                        if let Child::Tree(param_list) = &parameter_type_list.children[0] {
                            for child in &param_list.children {
                                match child {
                                    Child::Token(token) => {
                                        // Ensure that the token is a comma, if not then error
                                        if token.kind() != &TokenKind::COMMA {
                                            // TODO: Error handling
                                            tracing::error!(
                                                "{}",
                                                &format!(
                                                    "  {}  Unexpected node while lowering {}@{} \
                                                     to {}. Expected {} but found {}",
                                                    "PARSER".yellow(),
                                                    token.kind.to_string().green(),
                                                    token.span.to_string().black().italic(),
                                                    "Token".cyan(),
                                                    "COMMA".cyan(),
                                                    token.kind.to_string().green(),
                                                )
                                            );
                                        }
                                    }
                                    Child::Tree(param) => {
                                        let (param_type, param_name) =
                                            self.extract_parameter_info(param);
                                        params.push(Param {
                                            name: Symbol::from(param_name),
                                            ty:   param_type,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        (return_type, params, name)
    }

    // fn extract_function_signature(&self) -> (Box<DataType>, Vec<Param>, String) {
    //     let mut name = String::new();
    //     let mut params = vec![];
    //     let mut return_type = Box::new(DataType::Int);

    //     let decl_specifiers = &self.children[0];

    //     if let Child::Tree(decl_specifiers) = decl_specifiers {
    //         if decl_specifiers.kind == TreeKind::DeclarationSpecifiers {
    //             if let Child::Tree(type_specifier) = &decl_specifiers.children[0]
    // {                 if type_specifier.kind == TreeKind::TypeSpecifier {
    //                     return_type = Box::new(type_specifier.transform_type());
    //                 }
    //             }
    //         } else {
    //             // TODO: Error handling
    //             tracing::error!(
    //                 "Expected DeclarationSpecifiers node while lowering function
    // declaration"             );
    //         }
    //     }

    //     let declarator = self.get_declarator();

    //     if let Child::Tree(direct_declarator) = &declarator.children[0] {
    //         if direct_declarator.kind == TreeKind::DirectDeclarator {
    //             // Get the function name
    //             if let Child::Token(token) = &direct_declarator.children[0] {
    //                 if token.kind == TokenKind::IDENTIFIER {
    //                     name = token.lexeme.clone();
    //                 } else {
    //                     // TODO: Error handling
    //                     tracing::error!(
    //                         "Expected Identifier token while lowering function
    // declaration"                     );
    //                 }
    //             }

    //             // Get the function parameters (if any)
    //             if let Child::Tree(parameter_type_list) =
    // &direct_declarator.children[2] {                 if
    // parameter_type_list.kind == TreeKind::ParamTypeList {
    // if let Child::Tree(param_list) = &parameter_type_list.children[0] {
    //                         for child in &param_list.children {
    //                             println!("child {:#?}", child);
    //                             if let Child::Tree(param) = child {
    //                                 let (param_type, param_name) =
    //                                     self.extract_parameter_info(param);
    //                                 // let (param_type, param_name) =
    //                                 // self.extract_parameter_info(child);
    //                                 params.push(Param {
    //                                     name: Symbol::from(param_name),
    //                                     ty:   param_type,
    //                                 });
    //                             } else {
    //                                 // TODO: Error handling
    //                                 tracing::error!(
    //                                     "Expected ParameterDeclaration node while
    // lowering \                                      function declaration"
    //                                 );
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }

    //     (return_type, params, name)
    // }

    fn get_declarator(&self) -> &Tree {
        match &self.children[1] {
            Child::Tree(tree) if tree.kind == TreeKind::Declarator => tree,
            _ => panic!("Expected Declarator node in function declaration"),
        }
    }

    // fn extract_parameter_info(&self, param: &Tree) -> (DataType, String) {
    //     let mut param_type = DataType::Int;
    //     let mut param_name = String::new();

    //     if param.kind == TreeKind::ParameterDeclaration {
    //         if let Child::Tree(declaration_specifiers) = &param.children[0] {
    //             if let Child::Tree(type_specifier) =
    // &declaration_specifiers.children[0] {                 if
    // type_specifier.kind == TreeKind::TypeSpecifier {
    // param_type = type_specifier.transform_type();                 }
    //             }
    //         }

    //         if let Child::Tree(direct_declarator) = &param.children[1] {
    //             if let Child::Token(token) = &direct_declarator.children[0] {
    //                 param_name = token.lexeme.clone();
    //             }
    //         }
    //     }

    //     (param_type, param_name)
    // }

    fn extract_function_body(&self) -> Statement {
        // The function body should be the last child of the FunctionDef node.

        // The last child should be a Tree node with kind CompoundStatement.
        if let Some(last_child) = self.children.last() {
            if let Child::Tree(last_tree) = last_child {
                if last_tree.kind == TreeKind::CompoundStatement {
                    // println!("last_tree {:#?}", last_tree);
                    return last_tree.extract_compound_statement();
                }
            }
        } else {
            tracing::error!(
                "{}",
                &format!(
                    "  {}  Unexpected node while lowering {}@{} to {}. Expected {} but found {}",
                    "PARSER".yellow(),
                    self.kind.to_string().green(),
                    self.range.to_string().black().italic(),
                    "FunctionDef".cyan(),
                    "CompoundStatement".cyan(),
                    self.kind.to_string().green(),
                )
            );
        }

        Statement::Return(Expr::Literal(Literal::IntegerConstant(0)))
    }

    // This function recursively extracts a compound statement.
    fn extract_compound_statement(&self) -> Statement {
        // Reference:
        /// compound_statement
        /// : '{' '}'
        /// | '{' statement_list '}'
        /// | '{' declaration_list '}'
        /// | '{' declaration_list statement_list '}'
        /// ;
        let mut statements = Vec::<Statement>::new();

        println!("extract_compound_statement {:#?}", self);

        // if let Some(

        let mut compound_statement_seen = false;

        for child in &self.children {
            // Check if the current child is a Statement.
            if let Child::Tree(child) = child {
                // Process the block item list, if anyy.
                if child.kind == TreeKind::BlockItemList {
                    let statement = child.extract_block_item_list();
                    statements.push(statement);
                    compound_statement_seen = true;
                }
            }
        }

        if compound_statement_seen {
            println!("compound_statement_seen {:#?}", compound_statement_seen);
        }

        // Create and return a compound statement with the extracted statements.
        // todo!("Implement compound statement extraction")
        Statement::Compound(Block::from(statements))
    }

    // fn transform_function(&self) -> Function {
    //     // TODO: Need to refactor this code to
    //     // to be more modular and reusable

    //     let mut name = String::new();
    //     let mut params = vec![];
    //     let mut return_type = Box::new(DataType::Int);
    //     let mut body =
    // Statement::Return(Expr::Literal(Literal::IntegerConstant(0)));

    //     tracing::trace!(
    //         "{}",
    //         &format!(
    //             "{} Lowering {}@{} to {}",
    //             "PARSER".yellow(),
    //             "FunctionDef".green(),
    //             self.range.to_string().black().italic(),
    //             "Function".cyan()
    //         )
    //     );

    //     let decl_specifiers = &self.children[0];
    //     // println!("decl_specifiers {:#?}", decl_specifiers);
    //     // println!("declarator {:#?}", declarator);

    //     // Parse the declaration specifiers to get the return type
    //     if let Child::Tree(decl_specifiers) = decl_specifiers {
    //         if decl_specifiers.kind == TreeKind::DeclarationSpecifiers {
    //             if let Child::Tree(type_specifier) = &decl_specifiers.children[0]
    // {                 if type_specifier.kind == TreeKind::TypeSpecifier {
    //                     return_type = Box::new(type_specifier.transform_type());
    //                 }
    //             }
    //         } else {
    //             // TODO: Error handling
    //             tracing::error!(
    //                 "Expected DeclarationSpecifiers node while lowering function
    // declaration"             );
    //         }
    //     }

    //     // Extract the declarator node
    //     let declarator = match &self.children[1] {
    //         Child::Tree(tree) if tree.kind == TreeKind::Declarator => tree,
    //         _ => panic!("Expected Declarator node in function declaration"),
    //     };

    //     // println!("declarator {:#?}", declarator);

    //     // Extract the function name (identifier)
    //     if let Child::Tree(direct_declarator) = &declarator.children[0] {
    //         if direct_declarator.kind == TreeKind::DirectDeclarator {
    //             // Get the function name
    //             if let Child::Token(token) = &direct_declarator.children[0] {
    //                 if token.kind == TokenKind::IDENTIFIER {
    //                     tracing::trace!(
    //                         "{}",
    //                         &format!(
    //                             "{} Lowering {}@{} to {} {} {}{}{}",
    //                             "PARSER".yellow(),
    //                             token.kind.to_string().blue(),
    //                             token.span().to_string().black().italic(),
    //                             "FunctionName".cyan(),
    //                             "-".red(),
    //                             " ".yellow().on_black(),
    //                             token.lexeme.green().on_black(),
    //                             " ".yellow().on_black(),
    //                         )
    //                     );
    //                     name = token.lexeme.clone();
    //                 } else {
    //                     // TODO: Error handling
    //                     tracing::error!(
    //                         "Expected Identifier token while lowering function
    // declaration"                     );
    //                 }
    //             }

    //             // Get the function parameters (if any)
    //             if let Child::Tree(parameter_type_list) =
    // &direct_declarator.children[2] {                 if
    // parameter_type_list.kind == TreeKind::ParamTypeList {
    // if let Child::Tree(param_list) = &parameter_type_list.children[0] {
    //                         // Get the parameter list
    //                         if param_list.kind == TreeKind::ParamList {
    //                             // println!("param_list {param_list:#?}");

    //                             // Get the parameters
    //                             for child in &param_list.children {
    //                                 let mut param_type = DataType::Int;
    //                                 let mut param_name = String::new();

    //                                 if let Child::Tree(param) = child {
    //                                     if param.kind ==
    // TreeKind::ParameterDeclaration {
    // // println!("param {:#?}", param);

    //                                         // Get the parameter type
    //                                         if let
    // Child::Tree(declaration_specifiers) =
    // &param.children[0]                                         {
    //                                             // println!(
    //                                             //     "declaration_specifiers \
    //                                             //
    // {declaration_specifiers:#?}"
    // // );

    //                                             // Get the parameter type
    //                                             if let
    // Child::Tree(type_specifier) =
    // &declaration_specifiers.children[0]
    // {                                                 // TODO: Need to handle
    // case like char**                                                 // We
    // need to "follow the pointers" to get true
    // // parameter type

    //                                                 // TODO: Need to handle case
    // like char[10]

    //                                                 if type_specifier.kind ==
    //                                                     TreeKind::TypeSpecifier
    //                                                 {
    //                                                     // Get the parameter type
    //                                                     param_type =
    //
    // type_specifier.transform_type();
    // }                                             }
    //                                         }

    //                                         // Get the parameter name
    //                                         if let Child::Tree(direct_declarator)
    // =                                             &param.children[1]
    //                                         {
    //                                             // Get the parameter name
    //                                             if let
    // Child::Token(direct_declarator) =
    // &direct_declarator.children[0]
    // {                                                 tracing::trace!(
    //                                                     "{}",
    //                                                     &format!(
    //                                                         "{} Lowering {}@{} to
    // {} {} {}{}{}",
    // "PARSER".yellow(),
    // direct_declarator
    // .kind
    // .to_string()
    // .blue(),
    // direct_declarator
    // .span()
    // .to_string()
    // .black()
    // .italic(),
    // "ParamName".cyan(),
    // "-".red(),                                                         "
    // ".yellow().on_black(),
    // direct_declarator
    // .lexeme
    // .green()
    // .on_black(),                                                         "
    // ".yellow().on_black(),
    // )                                                 );
    //                                                 param_name =
    // direct_declarator.lexeme.clone();
    // }                                         }

    //                                         // Add the parameter to the list
    //                                         params.push(Param {
    //                                             name: Symbol::from(param_name),
    //                                             ty:   param_type,
    //                                         });
    //                                     }
    //                                 }
    //                                 //                     // Get the
    //                                 // parameter
    //                                 // name
    //                             }
    //                         }
    //                     } else {
    //                         // TODO: Error handling
    //                         tracing::error!(
    //                             "Expected ParamList node while lowering function
    // declaration"                         );
    //                     }
    //                 }
    //             }
    //         }
    //     }

    //     //     let declarator = match &self.children[1] {
    //     //     Child::Tree(tree) if tree.kind == TreeKind::Declarator => tree,
    //     //     _ => panic!("Expected Declarator node in function declaration"),
    //     // };

    //     // TODO: Parse the declarator to get the function name and parameters

    //     // Extract the function body
    //     // let compound_statement = match &self.children[2] {
    //     //     Child::Tree(tree) if tree.kind == TreeKind::CompoundStatement =>
    // tree,     //     _ => panic!("Expected CompoundStatement node in function
    // declaration"),     // };

    //     // println!("compound_statement {:#?}", compound_statement);

    //     Function { name, params, return_type, body }
    // }

    pub fn contains_errors(&self) -> bool {
        self.kind == TreeKind::ErrorTree ||
            self.children
                .iter()
                .any(|child| matches!(child, Child::Tree(tree) if tree.contains_errors()))
    }

    pub fn num_errors(&self) -> usize {
        if self.kind == TreeKind::ErrorTree {
            1
        } else {
            self.children
                .iter()
                .map(|child| match child {
                    Child::Tree(tree) => tree.num_errors(),
                    _ => 0,
                })
                .sum()
        }
    }

    fn transform_type(&self) -> DataType {
        // Extract the type specifier from the parse tree
        let type_specifier = &self.children[0];

        if let Child::Token(type_specifier) = type_specifier {
            // println!("type_specifier {:#?}", type_specifier);
            // Match the type specifier to determine the data type
            match type_specifier.kind {
                TokenKind::CHAR_KW => DataType::Char,
                TokenKind::INT_KW => DataType::Int,
                TokenKind::FLOAT_KW => DataType::Float,
                TokenKind::DOUBLE_KW => DataType::Double,
                // Add more cases for other data types as needed
                _ => DataType::Unknown, // Handle unknown or unsupported data types
            }
        } else {
            DataType::Unknown
        }
    }

    fn transform_declaration(&self) -> Declaration {
        todo!()
    }

    fn extract_statement(&self) -> Statement {
        todo!()
    }

    fn extract_block_item_list(&self) -> Statement {
        todo!()
    }

    pub(crate) fn is_function(&self) -> bool {
        self.kind == TreeKind::FunctionDef
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        self.print(&mut buf, 0);
        write!(f, "{buf}")
    }
}
