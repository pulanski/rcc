mod gpp;
use pretty_assertions_sorted::assert_eq;

#[derive(Debug, Display, Clone)]
enum Token {
    Directive(String),
    Identifier(String),
    Text(String),
}

// struct Preprocessor {
//   macros: HashMap<String, String>,
// }

// impl Preprocessor {
//   fn new() -> Self {
//     Preprocessor { macros: HashMap::new() }
//   }

//   fn process(&mut self, tokens: Vec<Token>) -> Vec<String> {
//     let mut result = Vec::new();
//     let mut token_iter = tokens.iter().peekable();

//     while let Some(token) = token_iter.next() {
//       match token {
//         Token::Directive(directive)
//           if directive == "#define" =>
//         {
//           if let Some(Token::Identifier(name)) =
//             token_iter.next()
//           {
//             if let Some(Token::Text(value)) = token_iter.next() {
//               self.macros.insert(name.clone(), value.clone());
//             }
//           }
//         }
//         Token::Directive(directive)
//           if directive == "#include" =>
//         {
//           if let Some(Token::Text(filename)) = token_iter.next()
//           {
//             if let Ok(content) = self.include_file(&filename) {
//               let sub_tokens = self.tokenize(&content);
//               let processed = self.process(sub_tokens);
//               result.extend(processed);
//             }
//           }
//         }
//         Token::Identifier(identifier) => {
//           if let Some(replacement) = self.macros.get(identifier)
//           {
//             result.push(replacement.clone());
//           } else {
//             result.push(identifier.clone());
//           }
//         }
//         Token::Text(text) => {
//           result.push(text.clone());
//         }
//         _ => {
//           // Other tokens are passed through as-is.
//         }
//       }
//     }

//     result
//   }

//   fn tokenize(&self, content: &str) -> Vec<Token> {
//     let mut tokens = Vec::new();
//     let mut buffer = String::new();
//     let mut in_directive = false;

//     for c in content.chars() {
//       if c.is_whitespace() {
//         if in_directive {
//           tokens.push(Token::Directive(buffer.clone()));
//         } else {
//           tokens.push(Token::Text(buffer.clone()));
//         }
//         buffer.clear();
//         in_directive = false;
//       } else if c == '#' && !in_directive {
//         in_directive = true;
//       }
//       buffer.push(c);
//     }

//     if !buffer.is_empty() {
//       if in_directive {
//         tokens.push(Token::Directive(buffer));
//       } else {
//         tokens.push(Token::Text(buffer));
//       }
//     }

//     tokens
//   }

//   fn include_file(&self, filename: &str) -> io::Result<String> {
//     let mut content = String::new();
//     let path = Path::new(filename);

//     if path.is_file() {
//       let mut file = fs::File::open(filename)?;
//       file.read_to_string(&mut content)?;
//     }

//     Ok(content)
//   }
// }

// pub(crate) fn preprocessor() {
//   let source_code = r#"
//         #define HELLO "Hello, World!"
//         int main() {
//             printf(HELLO);
//             return 0;
//         }
//     "#;

//   let mut preprocessor = Preprocessor::new();
//   let tokens = preprocessor.tokenize(source_code);
//   let processed = preprocessor.process(tokens);

//   for line in processed {
//     println!("{line}");
//   }
// }

use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use strum_macros::Display;

struct Preprocessor {
    macros: HashMap<String, String>,
    include_paths: Vec<PathBuf>,
}

impl Preprocessor {
    fn new() -> Self {
        Preprocessor { macros: HashMap::new(), include_paths: vec![PathBuf::from(".")] }
    }

    fn define_macro(&mut self, name: &str, value: &str) {
        self.macros.insert(name.to_string(), value.to_string());
    }

    fn process_file(&self, filename: &str) -> Result<String> {
        // let content = fs::read_to_string(filename)?;

        // TODO: Handle #include directives
        // process_include_directives(&content);
        // TODO: Handle #define directives
        // process_define_directives(&content);

        // Ok("".to_string())

        let content = fs::read_to_string(filename)?;
        println!("Content: {content}");

        let tokens = self.tokenize(&content);
        println!("Tokens: {tokens:?}");

        let processed_tokens = self.process(tokens);
        println!("Processed tokens: {processed_tokens:?}");

        let result = processed_tokens.join("");
        Ok(result)
    }

    fn process(&self, tokens: Vec<Token>) -> Vec<String> {
        // Implement this method to handle macro expansion and other processing.
        // We'll add the logic for handling #include and #define directives here.
        // For now, we'll return tokens as is.
        // You'll need to expand macros and handle includes in this function.
        tokens.iter().map(|t| t.to_string()).collect()
    }

    fn tokenize(&self, content: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut buffer = String::new();
        let mut in_directive = false;
        let re = Regex::new(r"\s+").unwrap();

        for c in content.chars() {
            if c == '#' && !in_directive {
                in_directive = true;
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.clone()));
                    buffer.clear();
                }
            }

            buffer.push(c);

            if c.is_whitespace() {
                if in_directive {
                    if !buffer.trim().is_empty() {
                        tokens.push(Token::Directive(buffer.clone()));
                    }
                } else if !buffer.trim().is_empty() {
                    tokens.push(Token::Text(buffer.clone()));
                }
                buffer.clear();
            }
        }

        if !buffer.is_empty() {
            if in_directive {
                tokens.push(Token::Directive(buffer));
            } else {
                tokens.push(Token::Text(buffer));
            }
        }

        tokens
    }

    fn include_file(&self, filename: &str) -> Result<String> {
        // Implement this method to handle file inclusion.
        // You'll need to search for the file in include paths and process it.
        Err(anyhow::anyhow!("Include file handling not implemented"))
    }
}

pub(crate) fn preprocessor() {
    let preprocessor = Preprocessor::new();

    // preprocessor.define_macro("PI", "3.14159");
    // preprocessor.define_macro("RADTODEG(x)", "((x) * 57.29578)");

    if let Ok(processed_content) = preprocessor.process_file("testdata/a.c") {
        println!("{processed_content}");
    } else {
        println!("Error processing file.");
    }
}

fn process_str(content: &str, context: &mut Preprocessor) -> Result<String> {
    let tokens = context.tokenize(content);
    let processed = context.process(tokens);
    Ok(processed.join(""))
}

// #[test]
// fn substitution() {
//   let mut context = Preprocessor::new();
//   context.macros.insert("Foo".to_string(), "Bar".to_string());

//   assert_eq!(process_str("Foo", &mut context).unwrap(), "Bar\n");
//   assert_eq!(
//     process_str("AFooB", &mut context).unwrap(),
//     "AFooB\n"
//   );
//   assert_eq!(
//     process_str("Foo_", &mut context).unwrap(),
//     "Foo_\n"
//   );
//   assert_eq!(
//     process_str("_Foo", &mut context).unwrap(),
//     "_Foo\n"
//   );
//   assert_eq!(
//     process_str("One Foo Two", &mut context).unwrap(),
//     "One Bar Two\n"
//   );
// }
