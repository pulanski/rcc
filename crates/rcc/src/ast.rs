use std::fmt::{self, Display};

use strum_macros::Display;

use crate::lexer::Token;

#[macro_export]
macro_rules! format_to {
    ($buf:expr) => ();
    ($buf:expr, $lit:literal $($arg:tt)*) => {
        { use ::std::fmt::Write as _; let _ = ::std::write!($buf, $lit $($arg)*); }
    };
}

#[derive(Debug, Display)]
pub enum TreeKind {
    TranslationUnit,
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
}

// impl Display for TreeKind {
//   fn fmt(
//     &self,
//     f: &mut std::fmt::Formatter<'_>,
//   ) -> std::fmt::Result {
//     match self {
//       TreeKind::TranslationUnit => write!(f, "TranslationUnit"),
//       TreeKind::StructDeclarator =>
//       TreeKind::EnumSpecifier =>
//       TreeKind::Enumerator => todo!(),
//       TreeKind::EnumeratorList => todo!(),
//       TreeKind::StructOrUnionSpecifier => todo!(),
//       TreeKind::PrimaryExpression => todo!(),
//       TreeKind::StructDeclaratorList => todo!(),
//       TreeKind::StructDeclaration => todo!(),
//       TreeKind::StructDeclarationList => todo!(),
//       TreeKind::ArgumentExpressionList => todo!(),
//       TreeKind::TypeName => todo!(),
//       TreeKind::SpecifierQualifierList => todo!(),
//       TreeKind::DirectDeclarator => todo!(),
//       TreeKind::ErrorTree => todo!(),
//       TreeKind::CompoundStatement => todo!(),
//       TreeKind::LogicalAndExpression => todo!(),
//       TreeKind::ExternDecl => todo!(),
//       TreeKind::File => todo!(),
//       TreeKind::PostfixExpression => todo!(),
//       TreeKind::InclusiveOrExpression => todo!(),
//       TreeKind::ExclusiveOrExpression => todo!(),
//       TreeKind::AndExpression => todo!(),
//       TreeKind::EqualityExpression => todo!(),
//       TreeKind::RelationalExpression => todo!(),
//       TreeKind::ShiftExpression => todo!(),
//       TreeKind::AdditiveExpression => todo!(),
//       TreeKind::MultiplicativeExpression => todo!(),
//       TreeKind::CastExpression => todo!(),
//       TreeKind::UnaryExpression => todo!(),
//       TreeKind::IdentifierList => todo!(),
//       TreeKind::StatementList => todo!(),
//       TreeKind::DirectAbstractDeclarator => todo!(),
//       TreeKind::Fn => todo!(),
//       TreeKind::TypeExpr => todo!(),
//       TreeKind::ParamList => todo!(),
//       TreeKind::LogicalOrExpression => todo!(),
//       TreeKind::Pointer => todo!(),
//       TreeKind::Declaration => todo!(),
//       TreeKind::DeclarationList => todo!(),
//       TreeKind::InitDeclaratorList => todo!(),
//       TreeKind::TypeQualifierList => todo!(),
//       TreeKind::InitDeclarator => todo!(),
//       TreeKind::Declarator => todo!(),
//       TreeKind::TypeSpecifier => todo!(),
//       TreeKind::TypeQualifier => todo!(),
//       TreeKind::Param => todo!(),
//       TreeKind::Block => todo!(),
//       TreeKind::StmtLet => todo!(),
//       TreeKind::StorageClassSpecifier => todo!(),
//       TreeKind::StmtReturn => todo!(),
//       TreeKind::StmtExpr => todo!(),
//       TreeKind::ExprLiteral => todo!(),
//       TreeKind::ExprName => todo!(),
//       TreeKind::ExprParen => todo!(),
//       TreeKind::ExprBinary => todo!(),
//       TreeKind::ExprCall => todo!(),
//       TreeKind::ArgList => todo!(),
//       TreeKind::Arg => todo!(),
//       TreeKind::DeclarationSpecifiers => todo!(),
//       TreeKind::FunctionDef => todo!(),
//     }
//   }
// }

#[derive(Debug)]
pub struct Tree {
    pub(crate) kind: TreeKind,
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
        format_to!(buf, "{indent}{:?}\n", self.kind);
        for child in &self.children {
            match child {
                Child::Token(token) => {
                    format_to!(
                        buf,
                        "{indent}  '{}'\n",
                        token.lexeme
                    )
                }
                Child::Tree(tree) => tree.print(buf, level + 1),
            }
        }
        assert!(buf.ends_with('\n'));
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        self.print(&mut buf, 0);
        write!(f, "{buf}")
    }
}
