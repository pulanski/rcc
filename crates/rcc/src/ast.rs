use crate::{
    cst::{
        Child,
        Tree,
    },
    diagnostics::{
        DiagnosticsEngine,
        FileId,
    },
};
use codespan_reporting::diagnostic::Diagnostic;
use derive_more::Display;
use owo_colors::OwoColorize;
use std::fmt::Display;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct AstSink {
    pub(crate) translation_unit: TranslationUnit,
    pub(crate) syntax_errors:    Vec<Diagnostic<FileId>>,
}

impl AstSink {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn num_errors(&self) -> usize {
        self.syntax_errors.len()
    }

    pub fn push_error(&mut self, error: Diagnostic<FileId>) {
        self.syntax_errors.push(error);
    }

    pub fn drain_errors(&mut self, diagnostics: &mut DiagnosticsEngine) {
        for error in self.syntax_errors.drain(..) {
            diagnostics.emit(error.clone());
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct TranslationUnit {
    pub functions: Vec<ExternDecl>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExternDecl {
    Function(Function),
    Declaration(Declaration),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    // pub name: Symbol,
    pub name:        Symbol,
    pub params:      Vec<Param>,
    pub return_type: Box<DataType>,
    pub body:        Statement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
    pub name: Symbol,
    pub ty:   DataType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Literal(Literal),
    Binary { left: Box<Expr>, operator: BinOp, right: Box<Expr> },
    // Add other types of expressions as needed
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    // Add other binary operators as needed
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Expression(Expr),
    If {
        condition:   Expr,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
    While {
        condition: Expr,
        body:      Vec<Statement>,
    },
    Return(Expr),
    Compound(Block),
    DoWhile {
        body:      Vec<Statement>,
        condition: Expr,
    },
    For {
        initializer: Option<Box<Statement>>,
        condition:   Option<Expr>,
        increment:   Option<Expr>,
        body:        Vec<Statement>,
    },
    Break,
    Continue,
    Assignment(Assignment),
}

impl Statement {
    pub fn is_compound(&self) -> bool {
        matches!(self, Statement::Compound(_))
    }

    pub fn has_statements(&self) -> bool {
        match self {
            Statement::Compound(block) => !block.statements.is_empty(),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

impl From<Vec<Statement>> for Block {
    fn from(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Identifier(String),
    IntegerConstant(i64),
    // Add other expression types like binary operations, function calls, etc.
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Expr>,
    // pub ret:  DataType, // TODO: do we need to add the return type?
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignment {
    pub left:  Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Constant {
    pub name: Symbol,
    pub ty:   DataType,
    pub val:  Expr,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Declaration {
    pub specifiers: Vec<DeclarationSpecifier>,
    pub ty:         DataType,
    pub var:        Symbol,
    // pub val: Option<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DeclarationSpecifier {
    Type(DataType),
    StorageClass(StorageClass),
    // Add other declaration specifiers as needed
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StorageClass {
    Extern,
    Static,
    Register,
    Auto,
    ThreadLocal,
    Typedef,
    // Add other storage classes as needed
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Enum {
    pub name:      Symbol,
    pub constants: Vec<Symbol>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Struct {
    pub name:    String,
    pub members: Vec<Declaration>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Type {
    pub ty: DataType,
}

#[derive(Debug, Display, PartialEq, Eq, Clone)]
#[display(fmt = "{name}")]
pub struct Symbol {
    pub name: String,
}

impl From<String> for Symbol {
    fn from(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DataType {
    // #[strum(serialize = "int")]
    Int,
    // #[strum(serialize = "char")]
    Char,
    // #[strum(serialize = "unknown")]
    Unknown,
    // #[strum(serialize = "float")]
    Float,
    // #[strum(serialize = "double")]
    Double,
    // #[strum(serialize = "pointer")]
    Pointer(Box<DataType>), // For pointer types
    // #[strum(serialize = "array")]
    Array(Box<DataType>, Option<usize>), // For array types with optional size (e.g. int[10])
    // #[strum(serialize = "struct")]
    Struct(Struct), // For struct types
    // #[strum(serialize = "enum")]
    Enum(Enum), // For enum types
    // #[strum(serialize = "function")]
    Function(Function), // For function types
    // #[strum(serialize = "void")]
    Void,
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Int => write!(f, "int"),
            DataType::Char => write!(f, "char"),
            DataType::Unknown => write!(f, "unknown"),
            DataType::Float => write!(f, "float"),
            DataType::Double => write!(f, "double"),
            DataType::Pointer(ty) => write!(f, "{ty}*"),
            DataType::Array(ty, size) => match size {
                Some(size) => write!(f, "{ty}[{size}]"),
                None => write!(f, "{ty}[]"),
            },
            DataType::Struct(s) => write!(f, "{}", s.name),
            DataType::Enum(e) => write!(f, "{}", e.name),
            DataType::Function(func) => write!(f, "{}", func.name),
            DataType::Void => write!(f, "void"),
        }
    }
}

/// Reduce a CST to an AST by lowering
pub fn reduce(tree: &mut Tree) -> AstSink {
    // pub fn reduce(tree: &mut Tree) -> TranslationUnit {
    tree.lower()
}

/// Reduce a CST to an AST by lowering and collecting diagnostics
pub fn reduce_with_diagnostics(
    tree: &mut Tree,
    diagnostics_engine: &mut crate::diagnostics::DiagnosticsEngine,
) -> AstSink {
    tree.lower_with_diagnostics(diagnostics_engine)
}

/// A visitor for lowering a CST to an AST. The visitor is responsible for
/// traversing the CST defined in a [`Tree`] and lowering it to an AST defined
pub struct Visitor {
    pub(crate) tree:    Tree,
    pub(crate) sink:    AstSink,
    pub(crate) file_id: FileId,
}

impl Visitor {
    /// Create a new visitor for lowering a CST to an AST
    pub fn new(tree: Tree, file_id: FileId) -> Self {
        Self { tree, sink: AstSink::new(), file_id }
    }

    /// Lower the CST to an AST
    pub fn lower(mut self) -> AstSink {
        self.visit_translation_unit();
        self.sink
    }

    /// Lower the CST to an AST and collect diagnostics
    pub fn lower_with_diagnostics(
        mut self,
        diagnostics_engine: &mut crate::diagnostics::DiagnosticsEngine,
    ) -> AstSink {
        self.visit_translation_unit();
        // self.sink.drain_errors(diagnostics_engine);
        self.sink
    }

    /// Visit the translation unit
    ///
    /// ```text
    /// translation_unit = external_declaration*
    /// ```
    ///
    /// # Examples
    ///
    /// ```text
    /// int main() {
    ///    return 0;
    /// }
    ///
    /// int foo() {
    ///   return 0;
    /// }
    /// ```
    fn visit_translation_unit(&mut self) {
        // println!("{:#?}", self.tree);

        let (num_functions, num_declarations) =
            (self.tree.num_functions(), self.tree.num_declarations());

        tracing::trace!(
            "{}",
            format!(
                "  {}  Lowering {} with {} {}{} and {} {}{}{}",
                "LOWERING".cyan(),
                "TranslationUnit".green(),
                num_functions,
                "function".blue(),
                if num_functions > 1 { "s".blue() } else { "".blue() },
                num_declarations,
                "declarations".yellow(),
                if num_declarations > 1 { "s".yellow() } else { "".yellow() },
                "...".black()
            )
        );

        // while !self.tree.is_empty() {
        //     self.visit_external_declaration();
        // }
    }

    fn visit_external_declaration(&self) {
        tracing::trace!(
            "{}",
            format!("  {}  Lowering {}...", "LOWERING".cyan(), "ExternalDeclaration".green())
        );

        let node = self.nth_child(0);

        // println!("{:#?}", node);

        // println!("{:#?}", self.tree);

        if self.tree.is_function() {
            println!("function");
            //     self.visit_function();
            // } else if self.tree.is_declaration() {
            //     self.visit_declaration();
        } else {
            tracing::error!(
                "{}",
                format!(
                    "  {}  {}",
                    "ERROR".red(),
                    "Expected a function or declaration while lowering".red()
                )
            );
        }
    }

    fn nth_child(&self, index: usize) -> Option<&Child> {
        self.tree.nth_child(index)
    }
}

pub fn lower_with_diagnostics(
    file_id: FileId,
    tree: Tree,
    diagnostics_engine: &mut DiagnosticsEngine,
) -> AstSink {
    Visitor::new(tree, file_id).lower_with_diagnostics(diagnostics_engine)
}
