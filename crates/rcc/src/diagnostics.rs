use crate::lexer::{
    Token,
    TokenKind,
};
use codespan_reporting::{
    diagnostic::{
        Diagnostic,
        Label,
    },
    files::{
        SimpleFile,
        SimpleFiles,
    },
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

#[derive(Debug, Getters, MutGetters, Setters, TypedBuilder)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct DiagnosticsEngine {
    pub diagnostics:      Vec<Diagnostic<usize>>,
    pub files:            SimpleFiles<String, String>,
    pub diagnostic_count: DiagnosticStats,
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

    pub fn emit(&mut self, diagnostic: Diagnostic<usize>) {
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

    pub fn emit_unexpected_token(
        &mut self,
        file_id: usize,
        unexpected_token: &Token,
        expected_token: &TokenKind,
    ) {
        self.emit(unexpected_token_diagnostic(file_id, unexpected_token, expected_token));
    }

    // pub fn push_diagnostic(&mut self, diagnostic: Diagnostic<usize>) {
    //     self.diagnostics.push(diagnostic);
    // }

    // pub fn emit_unterminated_string(&mut self, file_id: usize,
    // unterminated_string: &Token) {
    //     self.emit(unterminated_string_diagnostic(file_id, unterminated_string));
    // }

    // pub fn emit_unterminated_comment(&mut self, file_id: usize,
    // unterminated_comment: &Token) {
    //     self.emit(unterminated_comment_diagnostic(file_id,
    // unterminated_comment)); }

    // pub fn emit_unterminated_character(&mut self, file_id: usize,
    // unterminated_character: &Token) {
    //     self.emit(unterminated_character_diagnostic(file_id,
    // unterminated_character)); }

    // pub fn emit_unterminated_escape_sequence(
    //     &mut self,
    //     file_id: usize,
    //     unterminated_escape_sequence: &Token,
    // ) {
    //     self.emit(unterminated_escape_sequence_diagnostic(
    //         file_id,
    //         unterminated_escape_sequence,
    //     ));
    // }
}

pub(crate) fn unexpected_token_diagnostic(
    file_id: usize,
    unexpected_token: &Token,
    expected: &TokenKind,
) -> Diagnostic<usize> {
    Diagnostic::error()
        .with_code("E0001")
        .with_message(format!(
            "Unexpected token encountered{} {}{}{}{} {}{}{}{}{}",
            ":".black(),
            "'".cyan(),
            unexpected_token.lexeme.yellow(),
            "'".cyan(),
            ",".black(),
            "expected".blue(),
            ":".black(),
            "'".cyan(),
            expected.to_string().yellow(),
            "'".cyan(),
        ))
        .with_notes(
            vec![
                format!(
                    "The parser encountered an {}{} {}{}{}",
                    "unexpected token".red(),
                    ":".black(),
                    "'".cyan(),
                    unexpected_token.lexeme,
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
                    "Valid tokens should be used exclusively in the input{}",
                    ".".black()
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
