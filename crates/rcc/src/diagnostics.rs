use crate::{
    lexer::{
        Token,
        TokenKind,
    },
    parser::{
        display,
        FN_DEF_DECLARATION_SPECIFIERS_FIRST,
    },
};
pub use codespan_reporting::{
    diagnostic::{
        Diagnostic,
        Label,
    },
    files::SimpleFiles,
    term::{
        self,
        termcolor::{
            ColorChoice,
            StandardStream,
        },
    },
};
use getset::{
    Getters,
    MutGetters,
    Setters,
};
use owo_colors::OwoColorize;
use typed_builder::TypedBuilder;

pub type FileId = usize;

#[derive(Debug, Getters, MutGetters, Setters, TypedBuilder)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct DiagnosticsEngine {
    pub diagnostics:      Vec<Diagnostic<FileId>>,
    pub files:            SimpleFiles<String, String>,
    pub diagnostic_count: DiagnosticStats,
}

pub enum ErrorKind {
    Syntax(SyntaxError),
    Semantic(SemanticError),
    // Lowering(LoweringError),
}

pub enum SyntaxError {
    UnexpectedToken(UnexpectedToken),
    UnknownToken(Token),
    UnterminatedString(Token),
    UnterminatedComment(Token),
    UnterminatedCharacter(Token),
    UnterminatedEscapeSequence(Token),
    TypeSpecifierMissing(Token),
}

pub struct UnexpectedToken {
    pub unexpected_token: Token,
    pub expected:         TokenKind,
}

impl UnexpectedToken {
    pub fn new(unexpected_token: Token, expected: TokenKind) -> Self {
        Self { unexpected_token, expected }
    }
}

pub enum SemanticError {
    ExpectedStatement,
    ExpectedFunctionOrDecl,
}

#[derive(Debug, Default, Getters, MutGetters, Setters, TypedBuilder)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct DiagnosticStats {
    pub error_count:   usize,
    pub warning_count: usize,
    pub note_count:    usize,
}

impl DiagnosticStats {
    pub fn new() -> Self {
        Self::default()
    }
}

impl DiagnosticsEngine {
    pub fn new() -> Self {
        Self {
            diagnostics:      Vec::new(),
            files:            SimpleFiles::new(),
            diagnostic_count: DiagnosticStats::new(),
        }
    }

    pub fn emit(&mut self, diagnostic: Diagnostic<FileId>) {
        self.diagnostics.push(diagnostic);
    }

    pub fn add_file(
        &mut self,
        file_name: impl Into<String>,
        file_contents: impl Into<String>,
    ) -> usize {
        self.files.add(file_name.into(), file_contents.into())
    }

    pub fn flush(&mut self) {
        let mut writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();

        for diagnostic in self.diagnostics.drain(..) {
            term::emit(&mut writer, &config, &self.files, &diagnostic)
                .expect("Could not emit error");
        }
    }
}

pub(crate) fn unexpected_token_diagnostic(
    file_id: usize,
    unexpected_token: &Token,
    expected: &TokenKind,
) -> Diagnostic<FileId> {
    Diagnostic::error()
        .with_code("E0001")
        .with_message(format!(
            "Unexpected token encountered{} {}{}{}{}",
            ":".black(),
            "'".cyan(),
            unexpected_token.lexeme.yellow(),
            "'".cyan(),
            ".".black(),
        ))
        .with_notes(
            vec![
                format!(
                    "The parser encountered an {}{} {}{}{}",
                    "unexpected token".red(),
                    ":".black(),
                    "'".cyan(),
                    unexpected_token.lexeme.yellow(),
                    "'".cyan(),
                ),
                format!(
                    "It maybe useful to check the {}{} {}{}{}",
                    "token preceding".green(),
                    ":".black(),
                    "'".cyan(),
                    unexpected_token.lexeme.yellow(),
                    "'".cyan(),
                ),
            ]
            .into_iter()
            .collect(),
        )
        .with_labels(vec![
            Label::primary(file_id, *unexpected_token.span.start()..*unexpected_token.span.end())
                .with_message(format!(
                    "Unexpected token found here{} {}{}{}",
                    ":".black(),
                    "'".cyan(),
                    unexpected_token.lexeme.yellow(),
                    "'".cyan(),
                )),
            Label::secondary(file_id, *unexpected_token.span.start()..*unexpected_token.span.end())
                .with_message(format!(
                    "Expected token{} {}{}{}",
                    ":".black(),
                    "'".cyan(),
                    expected.to_string().green(),
                    "'".cyan(),
                )),
        ])
}

pub(crate) fn unknown_token_diagnostic(file_id: usize, unknown_token: &Token) -> Diagnostic<usize> {
    Diagnostic::error()
        .with_code("E0000")
        .with_message(format!(
            "Unknown token encountered{} {}{}{}",
            ":".black(),
            "'".cyan(),
            unknown_token.lexeme.yellow(),
            "'".cyan(),
        ))
        .with_notes(
            vec![
                format!(
                    "The lexer encountered an {}{} {}{}{}",
                    "unknown token".red(),
                    ":".black(),
                    "'".cyan(),
                    unknown_token.lexeme,
                    "'".cyan(),
                ),
                format!(
                    "This may be due to a {} or an {} in the input{}",
                    "typo".magenta(),
                    "unsupported character".magenta(),
                    ".".black()
                ),
                format!(
                    "Please check the input and make sure it contains {} {}{}",
                    "ONLY".blue(),
                    "supported tokens".green(),
                    ".".black()
                ),
                format!(
                    "For more information on {}{} please refer to the {}{}",
                    "supported tokens".green(),
                    ",".black(),
                    "C Language Specification".cyan(),
                    ".".black()
                ),
            ]
            .into_iter()
            .collect(),
        )
        .with_labels(vec![
            Label::primary(file_id, *unknown_token.span.start()..*unknown_token.span.end())
                .with_message(format!(
                    "Unknown token found here{} {}{}{}",
                    ":".black(),
                    "'".cyan(),
                    unknown_token.lexeme.yellow(),
                    "'".cyan(),
                )),
            Label::secondary(file_id, *unknown_token.span.start()..*unknown_token.span.end())
                .with_message(format!(
                    "Valid tokens should be used exclusively in the input{}",
                    ".".black()
                )),
        ])
}

pub(crate) fn expected_statement() -> Diagnostic<FileId> {
    // TODO: Implement this
    todo!()
}

pub(crate) fn expected_function_or_decl(file_id: usize, final_token: &Token) -> Diagnostic<FileId> {
    let mut diagnostic = Diagnostic::error().with_code("E0002").with_message(format!(
        "Expected {} or {}{} Instead found {}{}{}{}",
        "function".green(),
        "declaration".green(),
        ".".black(),
        "'".cyan(),
        "EOF".yellow(),
        "'".cyan(),
        ".".black(),
    ));

    // Add notes with tips and examples
    let mut notes = vec![
        format!(
            "You're missing a {} or a {} in your program{}\nHaving either a {} or a {} in your \
             program is {} to {}{}",
            "function definition".green(),
            "variable declaration".green(),
            ".".black(),
            "function definition".green(),
            "variable declaration".green(),
            " REQUIRED ".on_red(),
            "successfully compile".green(),
            ".".black(),
        ),
        format!(
                "{}{}\n\n{}{} {} a {} to your program{}\n\n{}{}\n\n{}\n{}\n{} {}{} {}, {} {}{} \
                 {}\n    {} x {} y;\n{}\n{}\n{}\n\n",
                "Tips for fixing this".yellow(),
                ":".black(),
                "1".magenta(),
                ".".black(),
                "Add".italic(),
                "function definition".green(),
                ".".black(),
                "Example".blue(),
                ":".black(),
                "```".black(),
                "+++++++++++".green(),
                "int".red(),
                "add".blue(),
                "(int".red(),
                "x".cyan(),
                "int".red(),
                "y".cyan(),
                ")".red(),
                "{".red(),
                "return".cyan(),
                "+".cyan(),
                "}".red(),
                "++++++++".green(),
                "```".black(),
            ),
        // TODO: Example 2: Declaration
        // "Example 2: Declaration".to_string(),
        // format!("  {} {} {};", "int".cyan(), "x".yellow(), "{}",),
        // "Ensure you have either a function definition or a declaration to start your
        // program."     .to_string(),
    ];

    diagnostic = diagnostic.with_notes(notes);

    // Add a label indicating the expected tokens
    diagnostic = diagnostic.with_labels(vec![Label::primary(
        file_id,
        *final_token.span().start()..*final_token.span().end(),
    )
    .with_message(format!(
        "Expected {}{}{}{}",
        "function".green(),
        " or ".black(),
        "declaration".green(),
        ".".black(),
    ))]);

    diagnostic

    // Diagnostic::error()
    //     .with_code("E0002")
    //     .with_message(format!(
    //         "Expected {} or {}{} Instead found {}{}{}{}",
    //         "function".green(),
    //         "declaration".green(),
    //         ".".black(),
    //         "'".cyan(),
    //         "EOF".yellow(),
    //         "'".cyan(),
    //         ".".black(),
    //     ))
    //     .with_notes(
    //         vec![
    //             // TODO: Add a note about adding either a function or a
    // declaration             // with some examples
    //         ]
    //         .into_iter()
    //         .collect(),
    //     )
    //     .with_labels(vec![Label::primary(
    //         file_id,
    //         *final_token.span().start()..*final_token.span().end(),
    //     )
    //     .with_message(format!(
    //         "Expected {}{}{}{}",
    //         "function".green(),
    //         " or ".black(),
    //         "declaration".green(),
    //         ".".black(),
    //     ))])
}

pub(crate) fn expected_declaration_specifier(
    file_id: usize,
    unexpected_token: &Token,
    expected_specifiers: &[TokenKind],
) -> Diagnostic<usize> {
    let expected_specifiers_str = expected_specifiers
        .iter()
        .map(|kind| format!("'{}'", kind.to_string().green()))
        .collect::<Vec<String>>()
        .join(", ");

    let mut diagnostic = Diagnostic::error()
        .with_code("E0004")
        .with_message(format!(
            "type specifier missing, defaults to 'int'; ISO C99 and later do not support implicit \
             int [-Wimplicit-int]"
        ))
        .with_notes(vec![
            format!(
                "The token '{}' is not a valid declaration specifier.",
                unexpected_token.lexeme.yellow()
            ),
            format!("Valid declaration specifiers include: {}", expected_specifiers_str),
            "Declaration specifiers determine the type of a declaration.".to_string(),
            "Ensure you use one of the valid declaration specifiers when declaring a function or \
             variable."
                .to_string(),
        ])
        .with_labels(vec![Label::primary(
            file_id,
            *unexpected_token.span.start()..*unexpected_token.span.end(),
        )
        .with_message(format!("Unexpected token '{}'", unexpected_token.lexeme.yellow()))]);

    // Add a label indicating the defaulting to 'int' behavior
    diagnostic = diagnostic.with_labels(vec![Label::secondary(
        file_id,
        *unexpected_token.span.start()..*unexpected_token.span.end(),
    )
    .with_message("Type defaults to 'int'")]);

    diagnostic
}

// pub(crate) fn type_specifier_missing(
//     file_id: FileId,
//     unexpected_token: &Token,
// ) -> Diagnostic<FileId> {
//     Diagnostic::error()
//         .with_code("E0005")
//         .with_message(format!(
//             "type specifier missing, defaults to 'int'; ISO C99 and later do
// not support implicit \              int [-Wimplicit-int]"
//         ))
//         .with_labels(vec![Label::primary(
//             file_id,
//             *unexpected_token.span.start()..*unexpected_token.span.end(),
//         )
//         .with_message("Type specifier missing, defaults to 'int'")])
//         .with_notes(vec![
//             format!(
//                 "The token '{}' is not a valid declaration specifier.",
//                 unexpected_token.lexeme.yellow()
//             ),
//             "Valid declaration specifiers include: 'void', 'char', 'short',
// 'int', 'long', \              'float', 'double', 'signed', 'unsigned',
// 'struct', 'union', 'enum', 'IDENTIFIER'"                 .to_string(),
//             "Declaration specifiers determine the type of a
// declaration.".to_string(),             "Ensure you use one of the valid
// declaration specifiers when declaring a function or \              variable."
//                 .to_string(),
//         ])
// }

pub(crate) fn type_specifier_missing(
    file_id: FileId,
    unexpected_token: &Token,
) -> Diagnostic<FileId> {
    let mut diagnostic = Diagnostic::error().with_code("E0005").with_message(format!(
        "{} {}{} defaults to {}{}{}{} {} and {} {} support {} {}{}{}",
        "Type specifier".cyan(),
        "missing".yellow().italic(),
        ",".black(),
        "'".green(),
        "int".magenta(),
        "'".green(),
        ";".black(),
        "ISO C99".blue(),
        "later".blue(),
        "DO NOT".red(),
        "implicit int".magenta(),
        "[".black(),
        "-Wimplicit-int".red(),
        "]".black(),
    ));

    // Add labels with semantic highlighting
    diagnostic = diagnostic.with_labels(vec![Label::primary(
        file_id,
        *unexpected_token.span.start()..*unexpected_token.span.end(),
    )
    .with_message(format!(
        // "Type specifier {}{} defaults to {}{}{}{}",
        "{} {}{} defaults to {}{}{}{}",
        "Type specifier".cyan(),
        "missing".yellow().italic(),
        ",".black(),
        "'".green(),
        "int".magenta(),
        "'".green(),
        ".".black(),
    ))]);

    // Add notes with semantic highlighting
    let notes = vec![
        format!(
            "The token {}{}{} is {} a valid {}{}\n\n{}{}\n\n{}\n\n",
            "'".cyan(),
            unexpected_token.lexeme.yellow(),
            "'".cyan(),
            "NOT".red(),
            "type specifier".cyan(),
            ".".black(),
            "Valid type specifiers include",
            ":".black(),
            display(FN_DEF_DECLARATION_SPECIFIERS_FIRST),
        ),
        format!(
            "{} determine the {} of a {} {}e{}g{} {}{} {};{}{} and are {} when {} a {} {} {}{}",
            "Type specifiers".cyan(),
            "type".magenta(),
            "declaration".green(),
            "(".black(),
            ".".black(),
            ".".black(),
            "`".yellow(),
            "int".red(),
            "x".cyan(),
            "`".yellow(),
            ")".black(),
            " REQUIRED ".on_red(),
            "declaring".green(),
            "function".green(),
            "or".black(),
            "variable".green(),
            ".".black(),
        ),
        format!(
            "Ensure you use one of the {} {} when {} a {} {} {}{}\n\n",
            "valid".green(),
            "type specifiers".cyan(),
            "declaring".green(),
            "function".green(),
            "or".black(),
            "variable".green(),
            ".".black(),
        ),
        format!(
            "For more information on {}{} please refer to the {}{}",
            "type specifiers".cyan(),
            ",".black(),
            "C Language Specification".cyan(),
            ".".black(),
        ),
        format!(
            "You can also use {}{} {}{} {}{} to get more information about {}{}",
            "`".yellow(),
            "rcc".green(),
            "--".black(),
            "explain".yellow(),
            "E0005".red(),
            "`".yellow(),
            "type specifiers".cyan(),
            ".".black(),
        ),
    ];

    diagnostic = diagnostic.with_notes(notes);

    diagnostic
}

// "Ensure you use one of the valid declaration specifiers when declaring a
// function or \  variable.\n\n"
//     .to_string(),
