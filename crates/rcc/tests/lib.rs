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
