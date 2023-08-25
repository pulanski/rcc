use crate::lexer::{Span, Token, TokenKind};
use owo_colors::OwoColorize;
use smartstring::alias::String;
use std::fmt::{self, Display};
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
}

#[derive(Debug)]
pub struct Tree {
    pub(crate) kind: TreeKind,
    pub(crate) range: Span,
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
    // AST nodes. AST nodes are a more minimal representation of thee
    // parse tree that is easier to work with, and is more suitable for
    // analysis and code generation.
    pub fn lower(&mut self) -> TranslationUnit {
        let mut functions = Vec::<ExternDecl>::new();
        for child in &mut self.children {
            if let Child::Tree(extern_decl) = child {
                // Go to the child node of the ExternDecl
                if extern_decl.kind == TreeKind::ExternDecl {
                    // Get the child node of the ExternDecl
                    let child = &mut extern_decl.children[0];

                    if let Child::Tree(child) = child {
                        if child.kind == TreeKind::FunctionDef {
                            functions.push(ExternDecl::Function(child.transform_function()));
                        }

                        // if child.kind == TreeKind::Declaration {
                        //     functions.push(ExternDecl::Declaration(child.
                        // transform_declaration())); }
                    }
                } else {
                    // TODO: Error handling
                }
            }
        }
        TranslationUnit { functions }
    }

    fn transform_function(&self) -> Function {
        // TODO: Need to refactor this code to
        // to be more modular and reusable

        let mut name = String::new();
        let mut params = vec![];
        let mut return_type = DataType::Int;
        let mut body = Statement::Return(Expression::Literal(Literal::IntegerConstant(0)));

        tracing::trace!(
            "{}",
            &format!(
                "{} Lowering {}@{} to {}",
                "PARSER".yellow(),
                "FunctionDef".green(),
                self.range.to_string().black().italic(),
                "Function".cyan()
            )
        );

        let decl_specifiers = &self.children[0];
        // println!("decl_specifiers {:#?}", decl_specifiers);
        // println!("declarator {:#?}", declarator);

        // Parse the declaration specifiers to get the return type
        if let Child::Tree(decl_specifiers) = decl_specifiers {
            if decl_specifiers.kind == TreeKind::DeclarationSpecifiers {
                if let Child::Tree(type_specifier) = &decl_specifiers.children[0] {
                    if type_specifier.kind == TreeKind::TypeSpecifier {
                        return_type = type_specifier.transform_type();
                    }
                }
            } else {
                // TODO: Error handling
                tracing::error!(
                    "Expected DeclarationSpecifiers node while lowering function declaration"
                );
            }
        }

        // Extract the declarator node
        let declarator = match &self.children[1] {
            Child::Tree(tree) if tree.kind == TreeKind::Declarator => tree,
            _ => panic!("Expected Declarator node in function declaration"),
        };

        // println!("declarator {:#?}", declarator);

        // Extract the function name (identifier)
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
                            // Get the parameter list
                            if param_list.kind == TreeKind::ParamList {
                                // println!("param_list {param_list:#?}");

                                // Get the parameters
                                for child in &param_list.children {
                                    let mut param_type = DataType::Int;
                                    let mut param_name = String::new();

                                    if let Child::Tree(param) = child {
                                        if param.kind == TreeKind::ParameterDeclaration {
                                            // println!("param {:#?}", param);

                                            // Get the parameter type
                                            if let Child::Tree(declaration_specifiers) =
                                                &param.children[0]
                                            {
                                                // println!(
                                                //     "declaration_specifiers \
                                                //      {declaration_specifiers:#?}"
                                                // );

                                                // Get the parameter type
                                                if let Child::Tree(type_specifier) =
                                                    &declaration_specifiers.children[0]
                                                {
                                                    // TODO: Need to handle case like char**
                                                    // We need to "follow the pointers" to get true
                                                    // parameter type

                                                    // TODO: Need to handle case like char[10]

                                                    if type_specifier.kind
                                                        == TreeKind::TypeSpecifier
                                                    {
                                                        // Get the parameter type
                                                        param_type =
                                                            type_specifier.transform_type();
                                                    }
                                                }
                                            }

                                            // Get the parameter name
                                            if let Child::Tree(direct_declarator) =
                                                &param.children[1]
                                            {
                                                // Get the parameter name
                                                if let Child::Token(direct_declarator) =
                                                    &direct_declarator.children[0]
                                                {
                                                    tracing::trace!(
                                                        "{}",
                                                        &format!(
                                                            "{} Lowering {}@{} to {} {} {}{}{}",
                                                            "PARSER".yellow(),
                                                            direct_declarator
                                                                .kind
                                                                .to_string()
                                                                .blue(),
                                                            direct_declarator
                                                                .span()
                                                                .to_string()
                                                                .black()
                                                                .italic(),
                                                            "ParamName".cyan(),
                                                            "-".red(),
                                                            " ".yellow().on_black(),
                                                            direct_declarator
                                                                .lexeme
                                                                .green()
                                                                .on_black(),
                                                            " ".yellow().on_black(),
                                                        )
                                                    );
                                                    param_name = direct_declarator.lexeme.clone();
                                                }
                                            }

                                            // Add the parameter to the list
                                            params.push(Parameter {
                                                name: param_name,
                                                ty: param_type,
                                            });
                                        }
                                    }
                                    //                     // Get the
                                    // parameter
                                    // name
                                }
                            }
                        } else {
                            // TODO: Error handling
                            tracing::error!(
                                "Expected ParamList node while lowering function declaration"
                            );
                        }
                    }
                }
            }
        }

        //     let declarator = match &self.children[1] {
        //     Child::Tree(tree) if tree.kind == TreeKind::Declarator => tree,
        //     _ => panic!("Expected Declarator node in function declaration"),
        // };

        // TODO: Parse the declarator to get the function name and parameters

        // Extract the function body
        // let compound_statement = match &self.children[2] {
        //     Child::Tree(tree) if tree.kind == TreeKind::CompoundStatement => tree,
        //     _ => panic!("Expected CompoundStatement node in function declaration"),
        // };

        // println!("compound_statement {:#?}", compound_statement);

        Function { name, params, return_type, body }
    }

    pub fn contains_errors(&self) -> bool {
        self.kind == TreeKind::ErrorTree
            || self
                .children
                .iter()
                .any(|child| matches!(child, Child::Tree(tree) if tree.contains_errors()))
    }

    fn transform_type(&self) -> DataType {
        // Extract the type specifier from the parse tree
        let type_specifier = &self.children[0];

        if let Child::Token(type_specifier) = type_specifier {
            // println!("type_specifier {:#?}", type_specifier);
            // Match the type specifier to determine the data type
            match type_specifier.kind {
                TokenKind::CHAR_KW => {
                    let datatype = DataType::Char;
                    tracing::trace!(
                        "{}",
                        &format!(
                            "{} Lowering {}@{} to {} {} {}",
                            "PARSER".yellow(),
                            self.kind.to_string().green(),
                            self.range.to_string().black().italic(),
                            "DataType".cyan(),
                            "-".red(),
                            datatype.magenta()
                        )
                    );

                    datatype
                }
                TokenKind::INT_KW => {
                    let datatype = DataType::Int;
                    tracing::trace!(
                        "{}",
                        &format!(
                            "{} Lowering {}@{} to {} {} {}",
                            "PARSER".yellow(),
                            self.kind.to_string().green(),
                            self.range.to_string().black().italic(),
                            "DataType".cyan(),
                            "-".red(),
                            datatype.magenta()
                        )
                    );

                    datatype
                }
                TokenKind::FLOAT_KW => {
                    let datatype = DataType::Float;

                    tracing::trace!(
                        "{}",
                        &format!(
                            "{} Lowering {}@{} to {} {} {}",
                            "PARSER".yellow(),
                            self.kind.to_string().green(),
                            self.range.to_string().black().italic(),
                            "DataType".cyan(),
                            "-".red(),
                            datatype.magenta()
                        )
                    );

                    datatype
                }
                TokenKind::DOUBLE_KW => {
                    let datatype = DataType::Double;
                    tracing::trace!(
                        "{}",
                        &format!(
                            "{} Lowering {}@{} to {} {} {}",
                            "PARSER".yellow(),
                            self.kind.to_string().green(),
                            self.range.to_string().black().italic(),
                            "DataType".cyan(),
                            "-".red(),
                            datatype.magenta()
                        )
                    );

                    datatype
                }
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
}

impl Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        self.print(&mut buf, 0);
        write!(f, "{buf}")
    }
}

pub fn reduce(tree: &mut Tree) -> TranslationUnit {
    tree.lower()
}

#[derive(Debug)]
pub struct TranslationUnit {
    pub functions: Vec<ExternDecl>,
}

#[derive(Debug)]
pub enum ExternDecl {
    Function(Function),
    Declaration(Declaration),
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: DataType,
    pub body: Statement,
}

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub ty: DataType,
}

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Binary { left: Box<Expression>, operator: String, right: Box<Expression> },
    // Add other types of expressions as needed
}

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
    If { condition: Expression, then_branch: Vec<Statement>, else_branch: Option<Vec<Statement>> },
    Return(Expression),
    // Add other statement types like loops, assignments, etc.
}

#[derive(Debug)]
pub enum Literal {
    Identifier(String),
    IntegerConstant(i64),
    // Add other expression types like binary operations, function calls, etc.
}

#[derive(Debug)]
pub struct Declaration {
    pub ty: DataType,
    pub var: Symbol,
    // pub val: Option<Expression>,
}

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
}

#[derive(Debug, Display, PartialEq, Eq, Clone, Copy)]
pub enum DataType {
    #[strum(serialize = "int")]
    Int,
    #[strum(serialize = "char")]
    Char,
    #[strum(serialize = "float")]
    Unknown,
    #[strum(serialize = "float")]
    Float,
    #[strum(serialize = "double")]
    Double,
    // Add more data types as needed
}
