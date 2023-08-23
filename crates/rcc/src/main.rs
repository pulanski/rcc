mod ast;
mod cst;
mod lexer;
mod parser;
mod preprocessor;
mod token_set;

use std::fs::read_to_string;

use ast::{Tree, TreeKind};
use lexer::{Token, TokenKind, TokenSink};
use logos::Logos;
use owo_colors::OwoColorize;
use parser::Parser;
use tracing_subscriber::fmt::Subscriber;

pub fn parse(text: &str) -> Tree {
  let token_sink: TokenSink = lex(text);
  let token_stream = token_sink.tokens;

  let mut p = Parser::new(token_stream);
  parser::translation_unit(&mut p);
  p.build_tree()
}

pub fn parse_tree(text: &str, tree_kind: TreeKind) -> Tree {
  let token_sink: TokenSink = lex(text);
  let token_stream = token_sink.tokens;

  let start = std::time::Instant::now();

  let mut p = Parser::new(token_stream);

  match tree_kind {
    TreeKind::TranslationUnit => todo!(),
    TreeKind::InitializerList => todo!(),
    TreeKind::ParameterList => todo!(),
    TreeKind::ParameterDeclaration => todo!(),
    TreeKind::Initializer => todo!(),
    TreeKind::StructDeclarator => todo!(),
    TreeKind::EnumSpecifier => todo!(),
    TreeKind::Enumerator => todo!(),
    TreeKind::Expression => todo!(),
    TreeKind::ConditionalExpression => todo!(),
    TreeKind::EnumeratorList => todo!(),
    TreeKind::StructOrUnionSpecifier => todo!(),
    TreeKind::PrimaryExpression => {
      parser::primary_expression(&mut p)
    }
    TreeKind::StructDeclaratorList => todo!(),
    TreeKind::StructDeclaration => todo!(),
    TreeKind::StructDeclarationList => todo!(),
    TreeKind::ArgumentExpressionList => todo!(),
    TreeKind::TypeName => todo!(),
    TreeKind::SpecifierQualifierList => todo!(),
    TreeKind::DirectDeclarator => todo!(),
    TreeKind::ErrorTree => todo!(),
    TreeKind::CompoundStatement => {
      parser::compound_statement(&mut p)
    }
    TreeKind::LogicalAndExpression => todo!(),
    TreeKind::ExternDecl => todo!(),
    TreeKind::File => todo!(),
    TreeKind::PostfixExpression => todo!(),
    TreeKind::InclusiveOrExpression => todo!(),
    TreeKind::ExclusiveOrExpression => todo!(),
    TreeKind::AndExpression => todo!(),
    TreeKind::EqualityExpression => todo!(),
    TreeKind::RelationalExpression => todo!(),
    TreeKind::ShiftExpression => todo!(),
    TreeKind::AdditiveExpression => todo!(),
    TreeKind::MultiplicativeExpression => todo!(),
    TreeKind::CastExpression => todo!(),
    TreeKind::UnaryExpression => {
      parser::unary_expression(&mut p)
    }
    TreeKind::IdentifierList => todo!(),
    TreeKind::StatementList => todo!(),
    TreeKind::DirectAbstractDeclarator => todo!(),
    TreeKind::Fn => todo!(),
    TreeKind::TypeExpr => todo!(),
    TreeKind::ParamList => todo!(),
    TreeKind::LogicalOrExpression => todo!(),
    TreeKind::Pointer => todo!(),
    TreeKind::Declaration => parser::declaration(&mut p),
    TreeKind::DeclarationList => todo!(),
    TreeKind::InitDeclaratorList => todo!(),
    TreeKind::TypeQualifierList => todo!(),
    TreeKind::InitDeclarator => todo!(),
    TreeKind::Declarator => todo!(),
    TreeKind::TypeSpecifier => todo!(),
    TreeKind::TypeQualifier => todo!(),
    TreeKind::Param => todo!(),
    TreeKind::Block => todo!(),
    TreeKind::StmtLet => todo!(),
    TreeKind::StorageClassSpecifier => todo!(),
    TreeKind::StmtReturn => todo!(),
    TreeKind::StmtExpr => todo!(),
    TreeKind::ExprLiteral => todo!(),
    TreeKind::ExprName => todo!(),
    TreeKind::ExprParen => todo!(),
    TreeKind::ExprBinary => todo!(),
    TreeKind::ExprCall => todo!(),
    TreeKind::ArgList => todo!(),
    TreeKind::Arg => todo!(),
    TreeKind::DeclarationSpecifiers => todo!(),
    TreeKind::FunctionDef => todo!(),
    TreeKind::UnaryOperator => todo!(),
    TreeKind::Statement => parser::statement(&mut p),
    TreeKind::LabeledStatement => todo!(),
    TreeKind::ExpressionStatement => todo!(),
    TreeKind::IterationStatement => todo!(),
    TreeKind::JumpStatement => parser::jump_statement(&mut p),
    TreeKind::SelectionStatement => todo!(),
    TreeKind::AssignmentExpression => todo!(),
    TreeKind::ConstantExpression => todo!(),
    TreeKind::FunctionSpecifier => todo!(),
    TreeKind::AlignmentSpecifier => todo!(),
  }

  let elapsed = start.elapsed();

  tracing::info!(
    " {}  {} {}{}{}{}",
    "PARSER".yellow(),
    " SUCCESS ".black().on_green(),
    "Parsing complete".italic(),
    " in ".black(),
    format!("{elapsed:?}").cyan(),
    ".".black()
  );

  p.build_tree()
}

pub fn lex(input: &str) -> TokenSink {
  let start = std::time::Instant::now();

  let mut lexer = TokenKind::lexer(input);
  let mut token_sink = TokenSink::new(input);
  // TokenSink::new(file_id, file.name().to_string().into());
  let mut current_unknown_token: Option<Token> = None;

  while let Some(token_result) = lexer.next() {
    match token_result {
      Ok(token) => {
        if let Some(unknown_token) =
          current_unknown_token.clone()
        {
          // TODO: Add diagnostic
          // token_sink.lexical_errors.push(
          //   unknown_token_diagnostic(
          //     file_id,
          //     &unknown_token,
          //   ),
          // );

          token_sink.tokens.push(unknown_token);
          current_unknown_token = None;
        }

        // If token is whitespace (e.g. a newline, comment, etc.), skip it.
        if token == TokenKind::WHITESPACE
          || token == TokenKind::COMMENT
          || token == TokenKind::NEWLINE
        {
          continue;
        }

        tracing::trace!(
          " {}  Creating token {} at {:?}",
          "LEXER".green(),
          token.yellow(),
          lexer.span().black().italic()
        );

        token_sink.tokens.push(Token::new(
          token,
          lexer.slice().to_string().into(),
          lexer.span().into(),
        ));
      }
      Err(()) => {
        if let Some(unknown_token) =
          current_unknown_token.clone()
        {
          let Token { kind: _, span, lexeme } = unknown_token;

          let span = span.merge(lexer.span());
          let updated_lexeme =
            format!("{}{}", lexeme, lexer.slice());

          tracing::debug!(
                            "Gluing together unknown tokens {} and {} to form {} at {}",
                            lexeme,
                            lexer.slice(),
                            updated_lexeme,
                            span
                        );

          current_unknown_token = Some(Token::new(
            TokenKind::UNKNOWN,
            updated_lexeme.into(),
            span,
          ));
        } else {
          tracing::debug!(
            "Creating new unknown token {} at {:?}",
            lexer.slice(),
            lexer.span()
          );

          current_unknown_token = Some(Token::new(
            TokenKind::UNKNOWN,
            lexer.slice().to_string().into(),
            lexer.span().into(),
          ));
        }
      }
    }
  }

  tracing::trace!(
    " {}  Creating token {} at {:?}",
    "LEXER".green(),
    "EOF".yellow(),
    lexer.span().black().italic()
  );

  token_sink.tokens.push(Token::new(
    TokenKind::EOF,
    "".to_string().into(),
    lexer.span().into(),
  ));

  // Pretty print the tokens.
  for (idx, token) in token_sink.tokens().iter().enumerate() {
    tracing::debug!(
      " {}  {} {} {} {}",
      "LEXER".green(),
      idx.to_string().red(),
      token,
      "->".yellow(),
      token.pretty_print()
    );
  }

  let elapsed = start.elapsed();

  tracing::info!(
    " {}   {} {}{}{}{}",
    "LEXER".yellow(),
    " SUCCESS ".black().on_green(),
    "Lexical analysis complete".italic(),
    " in ".black(),
    format!("{elapsed:?}").cyan(),
    ".".black()
  );

  token_sink
}

fn main() {
  let subscriber = Subscriber::builder()
    // .with_env_filter(EnvFilter::from_default_env())
    .with_ansi(true)
    .with_max_level(tracing::Level::DEBUG)
    .with_line_number(false)
    .with_thread_names(false)
    .without_time() // turn off timestamps
    .finish();

  // Set the subscriber as the default.
  tracing::subscriber::set_global_default(subscriber)
    .expect("failed to set subscriber");

  // let input = &read_to_string("testdata/a.c").unwrap();
  let input = "#include <stdio.h>\nint main() { printf(\"Hello, World!\"); return 0; }";
  let tree = parse(input);
  println!("{tree:?}");

  // preprocessor::preprocessor();

  let primary_expr_test_cases = vec![
    "x",         // Identifier
    "42",        // Constant
    "\"hello\"", // String Literal
    "(a + b)",   // Expression in parentheses
  ];

  // for input in primary_expr_test_cases {
  //   let cst = parse_tree(input, TreeKind::PrimaryExpression);
  //   println!("Parsing:\n\n{input}");
  //   eprintln!("\nTree:\n\n{cst}");
  // }

  let decl_test_cases = vec![
    // Declaration tests
    "int x;",       // Declaration with a single variable
    "int x, y, z;", // Declaration with multiple variables
    "extern int x = 42;", // Extern declaration with initialization
    "typedef int* IntPtr;", // Typedef declaration
    "struct Point { int x; int y; };", // Struct declaration
    "enum Color { RED, GREEN, BLUE };", // Enum declaration
    // Initializer tests
    "int x = 42;", // Declaration with initialization
    "int x[] = {1, 2, 3};", // Array initialization
                   // "struct Point p = {.x = 10, .y = 20};", // Struct initialization
  ];

  // for input in decl_test_cases {
  //   let cst = parse_tree(input, TreeKind::Declaration);
  //   println!("Parsing:\n\n{input}");
  //   println!("\nTree:\n\n{cst}");
  // }

  // fire emoji 🔥

  let statement_test_cases = [
    // Statement tests
    "goto label;",      // Goto statement
    "label: return 0;", // Labeled statement
    "x = 42;",          // Expression statement
    // Selection statement tests
    "if (x > 0) { return x; }", // If statement
    "if (x > 0) { return x; } else { return -x; }", // If-Else statement
    // Iteration statement tests
    "while (x > 0) { x--; }", // While loop
    "for (int i = 0; i < 10; i++) { printf(\"%d\\n\", i); }", // For loop
    "do { x--; } while (x > 0);", // Do-While loop
                                  // Jump statement tests

                                  //
                                  // "goto label; label: return 0;", // Goto statement
                                  // "switch (x) { case 1: y = 42; break; default: y = 0; }", // Switch statement
                                  // "while (x > 0) { x--; }", // While loop
                                  // "for (int i = 0; i < 10; i++) { printf(\"%d\\n\", i); }", // For loop
                                  // "do { x--; } while (x > 0);", // Do-While loop
                                  // "if (x > 0) { return x; }", // If statement with return
  ];

  // for input in statement_test_cases {
  //   let cst = parse_tree(input, TreeKind::Statement);
  //   println!("Parsing:\n\n{input}");
  //   println!("\nTree:\n\n{cst}");
  // }
}
