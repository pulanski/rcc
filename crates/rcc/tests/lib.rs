#[cfg(test)]
mod preprocessor_test_suite {
    use pretty_assertions_sorted::assert_eq;
    // use rcc;
    //   use rcc::preprocessor::Preprocessor;

    // TODO: Get tests working again within Buck
    //   #[test]
    //   fn test_syntax() {
    //     let source = r"def foo():
    //     pass
    // ";

    //   #[test]
    //   fn test_preprocessor() {
    //     let source = r"def foo():
    //     pass
    // ";

    //     let preprocessor =
    //       rcc::preprocessor::Preprocessor::new(source);
    //     let tokens = preprocessor.tokenize();
    //     println!("tokens: {:#?}", tokens);
    //     assert_eq!(
    //       tokens,
    //       vec!["def", "foo", "(", ")", ":", "pass"]
    //     );
    //   }
}

// v1
// #[cfg(test)]
// mod preprocessor_test_suite {
//   use pretty_assertions_sorted::assert_eq;
//   //   use rcc::preprocessor::Preprocessor;

//   //   #[test]
//   //   fn test_syntax() {
//   //     let source = r"def foo():
//   //     pass
//   // ";

//   #[test]
//   fn test_preprocessor() {
//     let source = r"def foo():
//     pass
// ";

//     // let preprocessor =
//     //   rcc::preprocessor::Preprocessor::new(source);
//     let tokens = preprocessor.tokenize();
//     println!("tokens: {:#?}", tokens);
//     assert_eq!(
//       tokens,
//       vec!["def", "foo", "(", ")", ":", "pass"]
//     );
//   }

//   //     // let expected
//   //     let ast = File::parse(source);
//   //     println!("ast: {ast:#?}");
//   //     assert_eq!(ast.debug_dump(), source);
//   //   }
// }

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Refactor to test all trees
    // #[rstest]
    // #[case("x", "Identifier")]
    // #[case("42", "Constant")]
    // #[case("\"hello\"", "String Literal")]
    // #[case("(a + b)", "Expression in parentheses")]
    // fn primary_expr_tests(input: &str, expected_result: &str) {
    //     let cst = parse_tree(input, TreeKind::PrimaryExpression);
    //     assert_eq!(cst, format!("Parsed PrimaryExpression:\n\n{}",
    // expected_result)); }
}

// // preprocessor::preprocessor();

// let primary_expr_test_cases = vec![
//     "x",         // Identifier
//     "42",        // Constant
//     "\"hello\"", // String Literal
//     "(a + b)",   // Expression in parentheses
// ];

// // for input in primary_expr_test_cases {
// //   let cst = parse_tree(input, TreeKind::PrimaryExpression);
// //   println!("Parsing:\n\n{input}");
// //   eprintln!("\nTree:\n\n{cst}");
// // }

// let decl_test_cases = vec![
//     // Declaration tests
//     "int x;",                           // Declaration with a single variable
//     "int x, y, z;",                     // Declaration with multiple
// variables     "extern int x = 42;",               // Extern declaration with
// initialization     "typedef int* IntPtr;",             // Typedef declaration
//     "struct Point { int x; int y; };",  // Struct declaration
//     "enum Color { RED, GREEN, BLUE };", // Enum declaration
//     // Initializer tests
//     "int x = 42;", // Declaration with initialization
//     "int x[] = {1, 2, 3};", /* Array initialization
//                     * "struct Point p = {.x = 10, .y = 20};", // Struct
//                       initialization */
// ];

// for input in decl_test_cases {
//   let cst = parse_tree(input, TreeKind::Declaration);
//   println!("Parsing:\n\n{input}");
//   println!("\nTree:\n\n{cst}");
// }

// fire emoji 🔥

// let statement_test_cases = [
//     // Statement tests
//     "goto label;",      // Goto statement
//     "label: return 0;", // Labeled statement
//     "x = 42;",          // Expression statement
//     // Selection statement tests
//     "if (x > 0) { return x; }",                     // If statement
//     "if (x > 0) { return x; } else { return -x; }", // If-Else statement
//     // Iteration statement tests
//     "while (x > 0) { x--; }", // While loop
//     "for (int i = 0; i < 10; i++) { printf(\"%d\\n\", i); }", // For loop
//     "do { x--; } while (x > 0);", // Do-While loop
//     // Jump statement tests
//     "goto label;", /* Goto statement */
//     "continue;",   /* Continue statement */
//     "break;",      /* Break statement */
//     "return 0;",   /* Return statement */
//     "return;",     /* Return statement without expression */
//     // Compound statement tests
//     "{ int x = 42; return x; }", // Compound statement
// ];

// for input in statement_test_cases {
//     let cst = parse_tree(input, TreeKind::Statement);
//     println!("Parsing:\n\n{input}");
//     println!("\nTree:\n\n{cst}");
// }

// parse the input file
// let input =
// &read_to_string("testdata/parse/translation_unit.c").unwrap();
// let tree = parse(input);
// println!("{tree:?}");

// Direct declarator tests
// let fn_def_test_cases = [
//     "int main() {
// int x;
// return 0;
//     }",
//     "int main(int argc, char** argv) { return 0; }",
//     "double add(double x, double y) { return x + y; }",
// ];

// for input in fn_def_test_cases {
//     let cst = parse_tree(input, TreeKind::TranslationUnit);

//     // println!("Parsing:\n\n{input}");
//     // println!("\nTree:\n\n{cst}");
// }
