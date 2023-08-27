use logos::{
    Lexer,
    Logos,
    Span,
};

#[derive(Logos, Debug, PartialEq)]
enum Token {
    // Define tokens for predefined macros
    #[regex(r"__FILE__")]
    File,

    #[regex(r"__LINE__")]
    Line,
}

fn line_number(span: Span, input: &str) -> usize {
    let mut line = 1;
    for (i, c) in input.char_indices() {
        if i >= span.start {
            break;
        }
        if c == '\n' {
            line += 1;
        }
    }
    line
}

fn replace_predefined_macros(input: &str) -> String {
    let mut output = String::new();
    let mut lexer = Token::lexer(input);

    while let Some(token_result) = lexer.next() {
        match token_result {
            Ok(Token::File) => {
                // Replace __FILE__ with the current input file name
                output.push_str("\"");
                const __FILE__: &str = env!("CARGO_MANIFEST_DIR");
                output.push_str(__FILE__);
                output.push_str("\"");
            }
            Ok(Token::Line) => {
                // Replace __LINE__ with the current line number
                output.push_str(&line_number(lexer.span(), input).to_string());
            }
            _ => {
                // Handle any other tokens or errors
                // For simplicity, just include the token as is
                output.push_str(lexer.slice());
            } /* Token::Error => {
               *     // Handle any other tokens or errors
               *     // For simplicity, just include the token as is
               *     output.push_str(lexer.slice());
               * } */
        }
    }

    output
}

fn main() {
    let input = "__FILE__ and
asdfasdf

asdfasdf


int main() {
    return
    __LINE__;
}



     __LINE__";
    let result = replace_predefined_macros(input);
    println!("Result: {}", result);
}
