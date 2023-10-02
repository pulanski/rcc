// use logos::{
//     Lexer,
//     Logos,
//     Span,
// };

// #[derive(Logos, Debug, PartialEq)]
// enum Token {
//     // Define tokens for predefined macros
//     #[regex(r"__FILE__")]
//     File,

//     #[regex(r"__LINE__")]
//     Line,
// }

// fn line_number(span: Span, input: &str) -> usize {
//     let mut line = 1;
//     for (i, c) in input.char_indices() {
//         if i >= span.start {
//             break;
//         }
//         if c == '\n' {
//             line += 1;
//         }
//     }
//     line
// }

// fn replace_predefined_macros(input: &str) -> String {
//     let mut output = String::new();
//     let mut lexer = Token::lexer(input);

//     while let Some(token_result) = lexer.next() {
//         match token_result {
//             Ok(Token::File) => {
//                 // Replace __FILE__ with the current input file name
//                 output.push_str("\"");
//                 const __FILE__: &str = env!("CARGO_MANIFEST_DIR");
//                 output.push_str(__FILE__);
//                 output.push_str("\"");
//             }
//             Ok(Token::Line) => {
//                 // Replace __LINE__ with the current line number
//                 output.push_str(&line_number(lexer.span(),
// input).to_string());             }
//             _ => {
//                 // Handle any other tokens or errors
//                 // For simplicity, just include the token as is
//                 output.push_str(lexer.slice());
//             } /* Token::Error => {
//                * // Handle any other tokens or errors
//                * // For simplicity, just include the token as is
//                * output.push_str(lexer.slice());
//                * } */
//         }
//     }

//     output
// }

// fn main() {
//     let input = "__FILE__ and
// asdfasdf

// asdfasdf

// int main() {
//     return
//     __LINE__;
// }

//      __LINE__";
//     let result = replace_predefined_macros(input);
//     println!("Result: {}", result);
// }

use std::fs::File;
use std::io;
use std::io::Write;
use std::process::Command;

// Define a struct to hold the macro and its value.
#[derive(Debug)]
pub struct MacroValue {
    pub macro_name: String,
    pub value:      String,
}

impl MacroValue {
    pub fn new(macro_name: &str, value: &str) -> Self {
        MacroValue { macro_name: macro_name.to_string(), value: value.to_string() }
    }
}

// Preprocess C code and retrieve macro values.
pub fn get_macro_values(c_code: &str, macros: &[&str]) -> io::Result<Vec<MacroValue>> {
    // Try to use Clang for preprocessing.
    let clang_result = preprocess_with_compiler("clang", c_code, macros);

    match clang_result {
        Ok(result) => Ok(result),
        Err(_) => {
            // If Clang is not found, try GCC.
            let gcc_result = preprocess_with_compiler("gcc", c_code, macros);

            match gcc_result {
                Ok(result) => Ok(result),
                Err(_) => {
                    // If neither Clang nor GCC is found, use default values.
                    let mut default_values = Vec::new();
                    for macro_name in macros {
                        default_values.push(MacroValue::new(macro_name, "default_value"));
                    }
                    Ok(default_values)
                }
            }
        }
    }
}

// Helper function to preprocess C code using a given compiler.
fn preprocess_with_compiler(
    compiler: &str,
    c_code: &str,
    macros: &[&str],
) -> io::Result<Vec<MacroValue>> {
    // Create a temporary C file.
    create_temp_c_file(c_code)?;

    // Build the command to preprocess the file.
    let mut cmd = Command::new(compiler);
    cmd.arg("-E").arg("temp.c").arg("-o").arg("temp.out");

    // Execute the command.
    let output = cmd.output();

    let out = std::fs::read_to_string("temp.out")?;
    // remove any lines starting with a hash sign

    let out = out.lines().filter(|line| !line.starts_with('#')).collect::<Vec<_>>().join("\n");

    println!("out: {}", out);

    let output = match output {
        Ok(output) => output,
        Err(e) => {
            println!("Error: {}", e);
            return Err(e);
        }
    };

    // Read the preprocessed output.
    let output = std::fs::read_to_string("temp.out")?;

    // Parse the macro values from the preprocessed output.
    let mut macro_values = Vec::new();
    for macro_name in macros {
        if let Some(value) = extract_macro_value(&output, macro_name) {
            macro_values.push(MacroValue::new(macro_name, &value));
        }
    }

    // Cleanup
    std::fs::remove_file("temp.c")?;
    std::fs::remove_file("temp.out")?;

    Ok(macro_values)
}

// Helper function to extract the value of a macro from preprocessed output.
fn extract_macro_value(output: &str, macro_name: &str) -> Option<String> {
    for line in output.lines() {
        if line.starts_with("#define") && line.contains(macro_name) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                return Some(parts[2].to_string());
            }
        }
    }
    None
}

// Helper function to create a temporary C file with the given code.
fn create_temp_c_file(c_code: &str) -> io::Result<()> {
    let mut file = File::create("temp.c")?;
    file.write_all(c_code.as_bytes())?;
    Ok(())
}

fn main() {
    let c_code = "__GNUC__\n__GNUC_MINOR__\n__STDC_VERSION__";
    let macros = vec!["__GNUC__", "__GNUC_MINOR__", "__STDC_VERSION__"];

    match get_macro_values(c_code, &macros) {
        Ok(values) => {
            for value in values {
                println!("{:?} = {}", value.macro_name, value.value);
            }
        }
        Err(_) => {
            println!("Failed to retrieve macro values.");
        }
    }
}
