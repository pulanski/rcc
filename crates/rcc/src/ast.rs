use crate::cst::{TranslationUnit, Tree};

/// Reduce a CST to an AST by lowering
pub fn reduce(tree: &mut Tree) -> TranslationUnit {
    tree.lower()
}
