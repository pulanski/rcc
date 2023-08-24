use crate::lexer::{
    Span,
    Token,
};
use owo_colors::OwoColorize;
use std::fmt::{
    self,
    Display,
};
use strum_macros::Display;

#[macro_export]
macro_rules! format_to {
    ($buf:expr) => ();
    ($buf:expr, $lit:literal $($arg:tt)*) => {
        { use ::std::fmt::Write as _; let _ = ::std::write!($buf, $lit $($arg)*); }
    };
}

#[derive(Debug, Display, PartialEq, Eq, Clone, Copy)]
pub enum TreeKind {
    TranslationUnit,
    StaticAssertDeclaration,
    AtomicTypeSpecifier,
    ExpressionStatement,
    IterationStatement,
    InitializerList,
    JumpStatement,
    UnaryOperator,
    ParameterList,
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
    ParamList,
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
    ParameterTypeList,
    StructOrUnion,
    AbstractDeclarator,
}

#[derive(Debug)]
pub struct Tree {
    pub(crate) kind:     TreeKind,
    pub(crate) range:    Span,
    pub(crate) children: Vec<Child>,
}

#[derive(Debug)]
pub enum Child {
    Token(Token),
    Tree(Tree),
}

impl Tree {
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
    // AST nodes.
    // pub fn transform(&mut self) {
    //     match self.kind {
    //         TreeKind::TranslationUnit => {
    //             // Transform the translation unit into a file node.
    //             self.kind = TreeKind::File;
    //         }

    pub fn contains_errors(&self) -> bool {
        self.kind == TreeKind::ErrorTree ||
            self.children
                .iter()
                .any(|child| matches!(child, Child::Tree(tree) if tree.contains_errors()))
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        self.print(&mut buf, 0);
        write!(f, "{buf}")
    }
}
