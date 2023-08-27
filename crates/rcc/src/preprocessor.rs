use crate::diagnostics::FileId;
use chrono::Datelike;
use chrono::Local;
use chrono::Timelike;
use codespan_reporting::diagnostic::Diagnostic;
use logos::{
    Logos,
    Span,
};
use owo_colors::OwoColorize;
use regex::Regex;
use std::fs::File;
use std::io::{
    self,
    BufRead,
    BufReader,
    Write,
};
use std::path::Path;
use std::time::Instant;
use walkdir::WalkDir;

pub struct Preprocessor {
    file_id:    FileId,
    input_text: String,
    output:     TextSink,
}

impl Preprocessor {
    pub fn new(file_id: FileId, input_text: String) -> Self {
        Self { file_id, input_text, output: TextSink::new() }
    }

    pub fn preprocess(&mut self) -> Result<(), Diagnostic<FileId>> {
        let text = &self.input_text;

        // Remove leading and trailing whitespace
        let text = text.strip_suffix("\r\n").or_else(|| text.strip_suffix('\n')).unwrap_or(text);

        // Perform global transformations on text (https://www.math.utah.edu/docs/info/cpp_1.html#SEC2)
        // let text = global_transform(text, self.file_id);

        for line in text.lines() {
            let line = line;
            let processed_line = process_line(line);
            self.output.transformed_text.push_str(&processed_line);
            self.output.transformed_text.push('\n');
        }

        Ok(())
    }

    pub fn into_output(self) -> TextSink {
        self.output
    }
}

pub struct TextSink {
    transformed_text: String,
    errors:           Vec<Diagnostic<FileId>>,
}

impl TextSink {
    pub fn new() -> Self {
        Self { transformed_text: String::new(), errors: Vec::new() }
    }

    pub fn num_errors(&self) -> usize {
        self.errors.len()
    }

    pub fn push_error(&mut self, error: Diagnostic<FileId>) {
        self.errors.push(error);
    }

    pub fn drain_errors(&mut self, diagnostics: &mut crate::diagnostics::DiagnosticsEngine) {
        for error in self.errors.drain(..) {
            diagnostics.emit(error.clone());
        }
    }

    pub fn into_transformed_text(self) -> String {
        self.transformed_text
    }
}

fn preprocess_file(input_path: &str, output_path: &str) -> io::Result<()> {
    let input_file = File::open(input_path)?;
    let output_file = File::create(output_path)?;

    let reader = BufReader::new(&input_file);
    let mut writer = io::BufWriter::new(output_file);

    let text = reader.lines().collect::<io::Result<Vec<String>>>()?.join("\n");

    // Remove leading and trailing whitespace
    let text =
        text.strip_suffix("\r\n").or_else(|| text.strip_suffix('\n')).unwrap_or(&text).to_string();

    // Perform global transformations on text (https://www.math.utah.edu/docs/info/cpp_1.html#SEC2)
    let text = global_transform(&text, Path::new(input_path));

    for line in text.lines() {
        let line = line;
        let processed_line = process_line(line);
        writer.write_all(processed_line.as_bytes())?;
        writer.write_all(b"\n")?;
    }

    Ok(())
}

/// Most C preprocessor features are inactive unless you give specific
/// directives to request their use. (Preprocessing directives are lines
/// starting with `#'; see section Preprocessing Directives). But there are
/// three transformations that the preprocessor always makes on all the input it
/// receives, even in the absence of directives.
///
/// All C comments are replaced with single spaces.
/// Backslash-Newline sequences are deleted, no matter where. This feature
/// allows you to break long lines for cosmetic purposes without changing their
/// meaning. Predefined macro names are replaced with their expansions (see
/// section Predefined Macros). The first two transformations are done before
/// nearly all other parsing and before preprocessing directives are recognized.
fn global_transform(text: &str, input_path: &Path) -> String {
    // Global transform: Remove comments.
    let text = remove_comments(text);

    // Delete Backslash-Newline sequences.
    let text = remove_backslash_newline(&text);

    // Replace predefined macro names with their expansions.
    replace_predefined_macros(&text, input_path)
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Logos, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    // Preprocessing directives and macros
    #[token("#")]
    HASH,
    #[token("__FILE__")]
    FILE,
    #[token("__LINE__")]
    LINE,
    #[token("__DATE__")]
    DATE,
    #[token("__TIME__")]
    TIME,
    #[token("__STDC__")]
    STDC,
    #[token("__STDC_VERSION__")]
    STDC_VERSION,

    // Punctuation
    #[token("+")]
    PLUS,
    #[token("-")]
    MINUS,
    #[token("*")]
    STAR,
    #[token("/")]
    SLASH,
    #[token("//", priority = 2)]
    DSLASH,
    #[token("%")]
    PERCENT,
    #[token("**")]
    DSTAR,
    #[token("~")]
    TILDE,
    #[token("&")]
    AMP,
    #[token("&&")]
    DOUBLEAMP,
    #[token("|")]
    PIPE,
    #[token("||")]
    DOUBLEPIPE,
    #[token("^")]
    CARET,
    #[token("<<")]
    LSHIFT,
    #[token(">>")]
    RSHIFT,
    #[token("=")]
    EQ,
    #[token("<")]
    LT,
    #[token(">")]
    GT,
    #[token(">=")]
    GE,
    #[token("<=")]
    LE,
    #[token("==")]
    EQEQ,
    #[token("!=")]
    NE,
    #[token("!")]
    BANG,
    #[token("+=")]
    PLUSEQ,
    #[token("-=")]
    MINUSEQ,
    #[token("*=")]
    STAREQ,
    #[token("/=")]
    SLASHEQ,
    #[token("//=")]
    DSLASHEQ,
    #[token("%=")]
    PERCENTEQ,
    #[token("&=")]
    AMPEQ,
    #[token("|=")]
    PIPEEQ,
    #[token("^=")]
    CARETEQ,
    #[token("<<=")]
    LSHIFTEQ,
    #[token(">>=")]
    RSHIFTEQ,
    #[token("?")]
    QUESTION,
    #[token("->")]
    PTR_OP,
    #[token("++")]
    INC_OP,
    #[token("--")]
    DEC_OP,
    #[token(".")]
    DOT,
    #[token(",")]
    COMMA,
    #[token(";")]
    SEMICOLON,
    #[token(":")]
    COLON,
    #[token("(")]
    LPAREN,
    #[token(")")]
    RPAREN,
    #[token("[")]
    LBRACKET,
    #[token("]")]
    RBRACKET,
    #[token("{")]
    LBRACE,
    #[token("}")]
    RBRACE,
    #[token("...")]
    ELLIPSIS,

    // // Constants
    // #[regex("0[xX][0-9a-fA-F]+")] // Hex Constant
    // #[regex("0[0-7]+(u|U|l|L)*")] // Octal Constant
    // #[regex("[0-9]+(u|U|l|L)*")] // Decimal Constant
    // #[regex("[0-9]*\\.[0-9]+([eE][+-]?[0-9]+)?(f|F|l|L)*")]
    // // Floating Constant
    // #[regex("'[^']*'")] // Character Constant
    // CONSTANT,

    // Integer constants
    // O   [0-7]
    // D   [0-9]
    // NZ  [1-9]
    // L   [a-zA-Z_]
    // A   [a-zA-Z_0-9]
    // H   [a-fA-F0-9]
    // HP  (0[xX])
    // E   ([Ee][+-]?{D}+)
    // P   ([Pp][+-]?{D}+)
    // FS  (f|F|l|L)
    // IS  (((u|U)(l|L|ll|LL)?)|((l|L|ll|LL)(u|U)?))
    // CP  (u|U|L)
    // SP  (u8|u|U|L)
    // ES  (\\(['"\?\\abfnrtv]|[0-7]{1,3}|x[a-fA-F0-9]+))
    // WS  [ \t\v\n\f]
    //
    // {HP}{H}+{IS}?				{ return I_CONSTANT; }
    // {NZ}{D}*{IS}?				{ return I_CONSTANT; }
    // "0"{O}*{IS}?				{ return I_CONSTANT; }
    // {CP}?"'"([^'\\\n]|{ES})+"'"		{ return I_CONSTANT; }
    #[regex("[xX][0-9a-fA-F]+(((u|U)(l|L|ll|LL)?)|((l|L|ll|LL)(u|U)?))?")]
    #[regex("[1-9][0-9]*(((u|U)(l|L|ll|LL)?)|((l|L|ll|LL)(u|U)?))?")]
    #[regex("0[0-7]*(((u|U)(l|L|ll|LL)?)|((l|L|ll|LL)(u|U)?))?")]
    // TODO: may need to come back to this
    // #[regex("(u|U|L)?'([^'\\\n]|(\\(['\"\\?\\abfnrtv]|[0-7]{1,3}|x[a-fA-F0-9]+)))*'")]
    INTEGER_CONSTANT,

    // {D}+{E}{FS}?				{ return F_CONSTANT; }
    // {D}*"."{D}+{E}?{FS}?			{ return F_CONSTANT; }
    // {D}+"."{E}?{FS}?			{ return F_CONSTANT; }
    // {HP}{H}+{P}{FS}?			{ return F_CONSTANT; }
    // {HP}{H}*"."{H}+{P}{FS}?			{ return F_CONSTANT; }
    // {HP}{H}+"."{P}{FS}?			{ return F_CONSTANT; }
    #[regex("[0-9]+[eE][+-]?[0-9]+(f|F|l|L)?")]
    #[regex("[0-9]*\\.[0-9]+([eE][+-]?[0-9]+)?(f|F|l|L)?")]
    #[regex("[0-9]+\\.([eE][+-]?[0-9]+)?(f|F|l|L)?")]
    #[regex("[xX][0-9a-fA-F]+[pP][+-]?[0-9]+(f|F|l|L)?")]
    #[regex("[xX][0-9a-fA-F]*\\.[0-9a-fA-F]+[pP][+-]?[0-9]+(f|F|l|L)?")]
    #[regex("[xX][0-9a-fA-F]+\\.[pP][+-]?[0-9]+(f|F|l|L)?")]
    FLOATING_CONSTANT,
    // Keywords
    // #[token("and")]
    // AND_KW,
    #[token("auto")]
    AUTO_KW,
    #[token("break")]
    BREAK_KW,
    #[token("case")]
    CASE_KW,
    #[token("char")]
    CHAR_KW,
    #[token("const")]
    CONST_KW,
    #[token("continue")]
    CONTINUE_KW,
    #[token("default")]
    DEFAULT_KW,
    #[token("do")]
    DO_KW,
    #[token("double")]
    DOUBLE_KW,
    #[token("else")]
    ELSE_KW,
    #[token("enum")]
    ENUM_KW,
    #[token("extern")]
    EXTERN_KW,
    #[token("float")]
    FLOAT_KW,
    #[token("for")]
    FOR_KW,
    #[token("goto")]
    GOTO_KW,
    #[token("if")]
    IF_KW,
    #[token("inline")]
    INLINE_KW,
    #[token("int")]
    INT_KW,
    #[token("long")]
    LONG_KW,
    #[token("register")]
    REGISTER_KW,
    #[token("restrict")]
    RESTRICT_KW,
    #[token("return")]
    RETURN_KW,
    #[token("short")]
    SHORT_KW,
    #[token("signed")]
    SIGNED_KW,
    #[token("sizeof")]
    SIZEOF_KW,
    #[token("static")]
    STATIC_KW,
    #[token("struct")]
    STRUCT_KW,
    #[token("switch")]
    SWITCH_KW,
    #[token("typedef")]
    TYPEDEF_KW,
    #[token("union")]
    UNION_KW,
    #[token("unsigned")]
    UNSIGNED_KW,
    #[token("void")]
    VOID_KW,
    #[token("volatile")]
    VOLATILE_KW,
    #[token("while")]
    WHILE_KW,
    #[token("_Alignas")]
    ALIGNAS_KW,
    #[token("_Alignof")]
    ALIGNOF_KW,
    #[token("_Atomic")]
    ATOMIC_KW,
    #[token("_Bool")]
    BOOL_KW,
    #[token("_Complex")]
    COMPLEX_KW,
    #[token("_Generic")]
    GENERIC_KW,
    #[token("_Imaginary")]
    IMAGINARY_KW,
    #[token("_Noreturn")]
    NORETURN_KW,
    #[token("_Static_assert")]
    STATIC_ASSERT_KW,
    #[token("_Thread_local")]
    THREAD_LOCAL_KW,
    #[token("__func__")]
    FUNC_NAME_KW,

    // 0[xX]{H}+{IS}?		{ count(); return(CONSTANT); }
    // 0{D}+{IS}?		{ count(); return(CONSTANT); }
    // {D}+{IS}?		{ count(); return(CONSTANT); }
    // L?'(\\.|[^\\'])+'	{ count(); return(CONSTANT); }

    // {D}+{E}{FS}?		{ count(); return(CONSTANT); }
    // {D}*"."{D}+({E})?{FS}?	{ count(); return(CONSTANT); }
    // {D}+"."{D}*({E})?{FS}?	{ count(); return(CONSTANT); }
    // #[regex("\\d+\\.\\d+([eE][\\+-]?\\d+)?", priority = 2)]
    // FLOAT_LITERAL,
    // #[regex("\\d+([eE][\\+-]?\\d+)?", priority = 2)]
    // INT_LITERAL,
    // #[regex("'(\\.|[^\\'])+'")]
    // CHAR_LITERAL,
    // #[regex("\"(\\.|[^\\\"])*\"")]
    // STRING_LITERAL,

    // #[token("load")]
    // LOAD_KW,
    // #[token("not")]
    // NOT_KW,
    // #[token("or")]
    // OR_KW,
    // #[token("def")]
    // DEF_KW,
    // #[token("in")]
    // IN_KW,
    // #[token("pass")]
    // PASS_KW,
    // #[token("elif")]
    // ELIF_KW,
    // #[token("lambda")]
    // LAMBDA_KW,

    // Identifiers and literals
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    IDENTIFIER,
    // #[regex("\\d+")]
    // INT,
    // // #[regex("(0x[0-9a-fA-F]+)|(0o[0-7]+)")]
    // // NumericLiteral,
    // #[regex("\\d*\\.?\\d+([eE][\\+-]?\\d+)?", priority = 2)]
    // FLOAT,
    // #[doc = LITERALS!("STRING")]
    #[regex(r#"b?"(\\.|[^\\"])*""#)]
    STRING,
    #[regex("b\"([^\"\\\\]|\\\\.)*\"|b'([^'\\\\]|\\\\.)*'")]
    BYTES,

    // #[doc = LITERALS!("RUNE")]
    // #[regex("b?'[^']*'")]
    // RUNE,

    // Whitespace and special tokens
    #[regex("//[^\n]*")]
    COMMENT,
    #[regex("[ \t]+")]
    WHITESPACE,
    #[regex("\r?\n")]
    NEWLINE,

    UNKNOWN,
    #[end]
    EOF,
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

fn current_date() -> String {
    let current_date = Local::now();
    let month = current_date.format("%b").to_string();
    let day = current_date.day();
    let year = current_date.year();

    format!("{month} {day} {year}")
}

fn current_time() -> String {
    let current_time = Local::now();
    let hour = current_time.hour();
    let minute = current_time.minute();
    let second = current_time.second();

    format!("{hour}:{minute}:{second}")
}

// This macro expands to the C Standard's version number, a long integer
// constant of the form `yyyymmL' where yyyy and mm are the year and month of
// the Standard version. This signifies which version of the C Standard the
// preprocessor conforms to. Like `__STDC__', whether this version number is
// accurate for the entire implementation depends on what C compiler will
// operate on the output from the preprocessor.
fn current_stdc_version() -> String {
    // TODO: implement
    todo!()
}

fn replace_predefined_macros(input: &str, input_path: &Path) -> String {
    tracing::trace!(
        "{}",
        &format!(
            "  {} Replacing {} in {}{}{}",
            "PREPROCESSOR".blue(),
            "predefined macros".green(),
            "'".cyan(),
            input_path.to_string_lossy().yellow(),
            "'".cyan()
        )
    );
    let mut output = String::new();
    let mut lexer = TokenKind::lexer(input);

    // println!("lexer: {:?}", lexer);

    while let Some(token_result) = lexer.next() {
        match token_result {
            Ok(TokenKind::FILE) => {
                // __FILE__
                // This macro expands to the name of the current input file, in the form of a C
                // string constant. The precise name returned is the one that was specified in
                // `#include' or as the input file name argument.
                tracing::trace!(
                    "{}",
                    &format!(
                        "  {}  {} {} {}{}{}",
                        "PREPROCESSOR".blue(),
                        "__FILE__".cyan(),
                        "->".black(),
                        "'".cyan(),
                        input_path.to_string_lossy().yellow(),
                        "'".cyan(),
                    )
                );
                output.push('\"');
                output.push_str(input_path.to_str().unwrap());
                output.push('\"');
            }
            Ok(TokenKind::LINE) => {
                // This macro expands to the current input line number, in the
                // form of a decimal integer constant. While we call it a predefined macro, it's
                // a pretty strange macro, since its "definition" changes with each new line of
                // source code. This and `__FILE__' are useful in generating an error message to
                // report an inconsistency detected by the program; the message can state the
                // source line at which the inconsistency was detected. For example,
                // fprintf (stderr, "Internal error: "
                //                  "negative string length "
                //                  "%d at %s, line %d.",
                //          length, __FILE__, __LINE__);
                // A `#include' directive changes the expansions of `__FILE__' and `__LINE__' to
                // correspond to the included file. At the end of that file, when processing
                // resumes on the input file that contained the `#include' directive, the
                // expansions of `__FILE__' and `__LINE__' revert to the values they had before
                // the `#include' (but `__LINE__' is then incremented by one as processing moves
                // to the line after the `#include'). The expansions of both `__FILE__' and
                // `__LINE__' are altered if a `#line' directive is used. See section Combining
                // Source Files.

                let line_number = line_number(lexer.span(), input);

                tracing::trace!(
                    "{}",
                    &format!(
                        "  {}  {} {} {}{}{}",
                        "PREPROCESSOR".blue(),
                        "__LINE__".cyan(),
                        "->".black(),
                        "'".cyan(),
                        line_number.to_string().red(),
                        "'".cyan(),
                    )
                );

                output.push_str(&line_number.to_string());
            }
            Ok(TokenKind::DATE) => {
                //  __DATE__
                // This macro expands to a string constant that describes the date on which the
                // preprocessor is being run. The string constant contains eleven characters and
                // looks like `"Jan 29 1987"' or `"Apr 1 1905"'.

                tracing::trace!(
                    "{}",
                    &format!(
                        "  {}  {} {} {}{}{}",
                        "PREPROCESSOR".blue(),
                        "__DATE__".cyan(),
                        "->".black(),
                        "'".cyan(),
                        current_date().magenta(),
                        "'".cyan(),
                    )
                );
                output.push('\"');
                output.push_str(&current_date());
                output.push('\"');
            }
            Ok(TokenKind::TIME) => {
                // __TIME__
                // This macro expands to a string constant that describes the time at which the
                // preprocessor is being run. The string constant contains eight characters and
                // looks like `"23:59:01"'.

                tracing::trace!(
                    "{}",
                    &format!(
                        "  {}  {} {} {}{}{}",
                        "PREPROCESSOR".blue(),
                        "__TIME__".cyan(),
                        "->".black(),
                        "'".cyan(),
                        current_time().magenta(),
                        "'".cyan(),
                    )
                );
                output.push('\"');
                output.push_str(&current_time());
                output.push('\"');
            }
            Ok(TokenKind::STDC) => {
                // __STDC__
                // This macro expands to the constant 1, to signify that this compiler conforms
                // to ISO Standard C. If a compiler doesn't conform to the
                // standard, it may not define this macro. If it does define
                // this macro, the value it defines it to is not useful; the
                // mere fact of defining this macro does not imply that the
                // compiler conforms. See section Alternate Preprocessing.

                tracing::trace!(
                    "{}",
                    &format!(
                        "  {}  {} {} {}{}{}",
                        "PREPROCESSOR".blue(),
                        "__STDC__".cyan(),
                        "->".black(),
                        "'".cyan(),
                        "1".red(),
                        "'".cyan(),
                    )
                );
                output.push('1');
            }
            Ok(TokenKind::STDC_VERSION) => {
                // __STDC_VERSION__
                // This macro expands to the C Standard's version number, a long
                // integer constant of the form `yyyymmL' where
                // yyyy and mm are the year and month of
                // the Standard version. This signifies which version of the C
                // Standard the preprocessor conforms to. Like
                // `__STDC__', whether this version number is
                // accurate for the entire implementation depends on what C
                // compiler will operate on the output from the
                // preprocessor.

                // tracing::trace!(
                //     "{}",
                //     &format!(
                //         "  {}  {} {} {}{}{}",
                //         "PREPROCESSOR".blue(),
                //         "__STDC_VERSION__".cyan(),
                //         "->".black(),
                //         "'".cyan(),
                //         "201710L".red(),
                //         "'".cyan(),
                //     )
                // );
            }
            Ok(_) => {
                // println!("replacing other: {:?}", lexer.slice());
                // Handle any other tokens or errors
                // For simplicity, just include the token as is
                output.push_str(lexer.slice());
            }
            Err(_) => {
                tracing::error!("Error replacing predefined macro: {:?}", lexer.slice());
                // Handle any other tokens or errors
                // For simplicity, just include the token as is
                output.push_str(lexer.slice());
            }
        }
    }

    output
}

fn process_line(line: &str) -> String {
    // Ignore empty lines
    if line.is_empty() {
        return line.to_string();
    }

    line.to_string()
}

fn remove_comments(input: &str) -> String {
    // Define a regular expression pattern to match C-style comments
    let pattern = Regex::new(r"/\*[\s\S]*?\*/|//.*").expect("Failed to compile regex pattern");

    // Use the replace_all method to replace comments with spaces
    let result = pattern.replace_all(input, " ");

    // Convert the regex::Captures to a String
    result.into_owned()
}

fn remove_backslash_newline(line: &str) -> String {
    // Define a regular expression pattern to match backslash-newline sequences
    let pattern = Regex::new(r"\\[\r\n]+").expect("Failed to compile regex pattern");

    // Use the replace method to remove backslash-newline sequences
    let result = pattern.replace_all(line, "");

    // Convert the regex::Captures to a String
    result.into_owned()
}

fn preprocess_files_recursively(root_dir: &str) -> io::Result<()> {
    for entry in WalkDir::new(root_dir) {
        let entry = entry?;
        if entry.file_type().is_file() {
            // If the file is a C file with the name `in.c`, preprocess it
            // and write the result to a file with the name `out.c`. Otherwise,
            // ignore the file.
            if let Some(extension) = entry.path().extension() {
                if extension == "c" && entry.file_name() == "in.c" {
                    let input_path = entry.path();
                    let output_path = input_path.with_file_name("out.c");

                    // println!("Preprocessing {}...", input_path.to_string_lossy());
                    // println!("  Writing to {}...", output_path.to_string_lossy());

                    let start = Instant::now();

                    tracing::debug!(
                        "{}",
                        &format!(
                            "  {} Preprocessing {}{}{}",
                            "PREPROCESSOR".blue(),
                            "'".cyan(),
                            input_path.to_string_lossy().yellow(),
                            "'".cyan()
                        )
                    );

                    preprocess_file(input_path.to_str().unwrap(), output_path.to_str().unwrap())?;

                    let elapsed = start.elapsed();

                    tracing::info!(
                        "  {}   {} {} completed{}{}{}",
                        "PREPROCESSOR".blue(),
                        " SUCCESS ".black().on_green(),
                        "Preprocessing".black(),
                        " in ".black(),
                        format!("{elapsed:?}").cyan(),
                        ".".black()
                    );
                }
            }
        }
    }
    Ok(())
}

pub fn main() {
    if let Err(err) = preprocess_files_recursively("testdata/preprocessor") {
        eprintln!("Error: {err}");
    } else {
        println!("Preprocessing completed successfully!");
    }
}
