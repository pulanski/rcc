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
use std::io::Read;
use std::io::{
    self,
    BufRead,
    BufReader,
    Write,
};
use std::path::Path;
use std::process::Command;
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

        let include_directories =
            vec![String::from("/usr/include"), String::from("/usr/local/include")];

        for line in text.lines() {
            let line = line;
            let processed_line = process_line(line, include_directories.as_slice());
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

fn preprocess_file(
    input_path: &str,
    output_path: &str,
    include_directories: &[String],
) -> io::Result<()> {
    let input_file = File::open(input_path)?;
    let output_file = File::create(output_path)?;

    let reader = BufReader::new(&input_file);
    let mut writer = io::BufWriter::new(output_file);

    let text = reader.lines().collect::<io::Result<Vec<String>>>()?.join("\n");

    // Remove leading and trailing whitespace
    let text = remove_leading_and_trailing_whitespace(text, input_path);

    // Perform global transformations on text (https://www.math.utah.edu/docs/info/cpp_1.html#SEC2)
    let text = global_transform(&text, Path::new(input_path));

    for line in text.lines() {
        let line = line;
        let processed_line = process_line(line, include_directories);
        writer.write_all(processed_line.as_bytes())?;
        writer.write_all(b"\n")?;
    }

    Ok(())
}

fn remove_leading_and_trailing_whitespace(text: String, input_path: &str) -> String {
    tracing::debug!(
        "{}",
        &format!(
            "  {} Removing {} from {}{}{}",
            "PREPROCESSOR".blue(),
            "leading and trailing whitespace".green(),
            "'".cyan(),
            input_path.yellow(),
            "'".cyan()
        )
    );
    let text =
        text.strip_suffix("\r\n").or_else(|| text.strip_suffix('\n')).unwrap_or(&text).to_string();
    text
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
    let text = replace_predefined_macros(&text, input_path);

    // TODO: Replace non-standard macros with their expansions.
    // https://www.math.utah.edu/docs/info/cpp_1.html#SEC15
    // let text = replace_non_standard_macros(&text);

    text
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
    #[token("__GNUC__")]
    GNUC,
    #[token("__GNUC_MINOR__")]
    GNUC_MINOR,
    #[token("__BASE_FILE__")]
    BASEFILE,
    #[token("__INCLUDE_LEVEL__")]
    INCLUDE_LEVEL,
    #[token("#include")]
    INCLUDE,

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

fn create_temp_c_file(c_code: &str) -> io::Result<()> {
    let mut file = File::create("temp.c")?;
    file.write_all(c_code.as_bytes())?;
    Ok(())
}

fn preprocess_stdc_version_with_clang() -> io::Result<String> {
    create_temp_c_file("__STDC_VERSION__")?;

    let output = Command::new("clang").arg("-E").arg("temp.c").arg("-o").arg("temp.out").output();

    let output = match output {
        Ok(output) => output,
        Err(e) => {
            println!("Error: {e}");
            return Err(e);
        }
    };

    // Remove any lines that start with a hash sign
    let output = std::fs::read_to_string("temp.out")?;
    let output =
        output.lines().filter(|line| !line.starts_with('#')).collect::<Vec<_>>().join("\n");

    // Cleanup
    std::fs::remove_file("temp.c")?;
    std::fs::remove_file("temp.out")?;

    Ok(output)
}

// Define a struct to hold the macro and its value.
#[derive(Debug)]
pub struct MacroValue {
    pub macro_name: String,
    pub value:      String,
}

// pub struct PreprocessorContext {
//     pub macros: HashSet<MacroValue>,
// }

impl MacroValue {
    pub fn new(macro_name: &str, value: &str) -> Self {
        MacroValue { macro_name: macro_name.to_string(), value: value.to_string() }
    }
}

// Preprocess C code and retrieve macro values.
pub fn get_macro_values(macros: &[&str]) -> io::Result<Vec<MacroValue>> {
    // Create C code which is simply a line for each macro.
    // e.g.
    // __STDC_VERSION__
    // __GNUC__
    // __GNUC_MINOR__
    let mut c_code = String::new();
    for macro_name in macros {
        c_code.push_str(macro_name);
        c_code.push('\n');
    }

    // Try to use Clang for preprocessing.
    let clang_result = preprocess_with_compiler("clang", &c_code, macros);

    match clang_result {
        Ok(result) => Ok(result),
        Err(_) => {
            // If Clang is not found, try GCC.
            let gcc_result = preprocess_with_compiler("gcc", &c_code, macros);

            match gcc_result {
                Ok(result) => Ok(result),
                Err(_) => {
                    // If neither Clang nor GCC is found, use default values.
                    // let mut default_values = Vec::new();
                    // for macro_name in macros {
                    //     default_values.push(MacroValue::new(macro_name, "default_value"));
                    // }
                    Ok(Vec::new())
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

    // Remove any lines starting with a hash sign
    let compiler_out = out.lines().filter(|line| !line.starts_with('#')).collect::<Vec<&str>>();

    // Parse the macro values from the preprocessed output.
    let mut macro_values = Vec::new();
    for (out_idx, macro_name) in macros.iter().enumerate() {
        if let Some(value) = extract_macro_value(compiler_out[out_idx], macro_name) {
            macro_values.push(MacroValue::new(macro_name, &value));

            // println!("{macro_name} = {value}");
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

    // Predefined macros are not defined by the preprocessor, so we need to
    // extract their values from the compiler.
    match macro_name {
        __GNUC__ | __GNUC_MINOR__ | __STDC_VERSION__ => Some(output.to_string()),
        _ => None,
    }
}

// This macro expands to the C Standard's version number, a long integer
// constant of the form `yyyymmL' where yyyy and mm are the year and month of
// the Standard version. This signifies which version of the C Standard the
// preprocessor conforms to. Like `__STDC__', whether this version number is
// accurate for the entire implementation depends on what C compiler will
// operate on the output from the preprocessor.
fn current_stdc_version() -> Option<String> {
    match preprocess_stdc_version_with_clang() {
        Ok(output) => Some(output),
        _ => None,
    }
}

const __FILE__: &str = "__FILE__";
const __LINE__: &str = "__LINE__";
const __DATE__: &str = "__DATE__";
const __TIME__: &str = "__TIME__";
const __STDC__: &str = "__STDC__";
const __GNUC__: &str = "__GNUC__";
const __GNUC_MINOR__: &str = "__GNUC_MINOR__";
const __STDC_VERSION__: &str = "__STDC_VERSION__";
const __BASE_FILE__: &str = "__BASE_FILE__";
const __INCLUDE_LEVEL__: &str = "__INCLUDE_LEVEL__";

const DEFAULT_STDC: &str = "1";
const DEFAULT_STDC_VERSION: &str = "201710L";
const DEFAULT_GNUC: &str = "4";
const DEFAULT_GNUC_MINOR: &str = "2";

fn replace_predefined_macros(input: &str, input_path: &Path) -> String {
    let mut include_level = 0; // Initialize the include level counter

    tracing::debug!(
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

    // Extract the values of certain predefined macros from
    // other host C compilers. This is meant for both testing purposes
    // as well as to drive the implementation of the preprocessor over
    // certain non-standard features, or non-trivial features to implement.
    let macros = vec![__GNUC__, __GNUC_MINOR__, __STDC_VERSION__];

    let macro_values = match get_macro_values(&macros) {
        Ok(values) => values,
        Err(_) => Vec::new(),
    };

    while let Some(token_result) = lexer.next() {
        match token_result {
            Ok(TokenKind::FILE) => {
                // __FILE__
                // This macro expands to the name of the current input file, in the form of a C
                // string constant. The precise name returned is the one that was specified in
                // `#include' or as the input file name argument.
                tracing::debug!(
                    "{}",
                    &format!(
                        "  {}  {} {} {}{}{}",
                        "PREPROCESSOR".blue(),
                        __FILE__.cyan(),
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
            Ok(TokenKind::INCLUDE) => {
                // Handle #include directive
                include_level += 1; // Increment include level

                output.push_str("#include");
            }
            Ok(TokenKind::EOF) => {
                // Handle end of file
                include_level -= 1; // Decrement include level
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

                tracing::debug!(
                    "{}",
                    &format!(
                        "  {}  {} {} {}{}{}",
                        "PREPROCESSOR".blue(),
                        __LINE__.cyan(),
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

                tracing::debug!(
                    "{}",
                    &format!(
                        "  {}  {} {} {}{}{}",
                        "PREPROCESSOR".blue(),
                        __DATE__.cyan(),
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

                tracing::debug!(
                    "{}",
                    &format!(
                        "  {}  {} {} {}{}{}",
                        "PREPROCESSOR".blue(),
                        __TIME__.cyan(),
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

                let stdc = macro_values
                    .iter()
                    .find(|m| m.macro_name == __STDC__)
                    .map(|m| m.value.clone())
                    .unwrap_or(DEFAULT_STDC.to_owned());

                tracing::debug!(
                    "{}",
                    &format!(
                        "  {}  {} {} {}{}{}",
                        "PREPROCESSOR".blue(),
                        __STDC__.cyan(),
                        "->".black(),
                        "'".cyan(),
                        stdc.red(),
                        "'".cyan(),
                    )
                );

                output.push_str(&stdc);
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

                let stdc_version = &macro_values
                    .iter()
                    .find(|m| m.macro_name == __STDC_VERSION__)
                    .map(|m| m.value.clone())
                    .unwrap_or(DEFAULT_STDC_VERSION.to_owned());

                tracing::debug!(
                    "{}",
                    &format!(
                        "  {}  {} {} {}{}{}",
                        "PREPROCESSOR".blue(),
                        __STDC_VERSION__.cyan(),
                        "->".black(),
                        "'".cyan(),
                        stdc_version.red(),
                        "'".cyan(),
                    )
                );

                output.push_str(stdc_version);
            }
            Ok(TokenKind::GNUC) => {
                // __GNUC__
                // This macro is defined if and only if this is GNU C. This
                // macro is defined only when the entire GNU C compiler is in
                // use; if you invoke the preprocessor directly, `__GNUC__' is
                // undefined. The value identifies the major version number of
                // GNU CC (`1' for GNU CC version 1, which is now obsolete, and
                // `2' for version 2).

                let gnu_c = &macro_values
                    .iter()
                    .find(|m| m.macro_name == __GNUC__)
                    .map(|m| m.value.clone())
                    .unwrap_or(DEFAULT_GNUC.to_owned());

                tracing::debug!(
                    "{}",
                    &format!(
                        "  {}  {} {} {}{}{}",
                        "PREPROCESSOR".blue(),
                        __GNUC__.cyan(),
                        "->".black(),
                        "'".cyan(),
                        gnu_c.red(),
                        "'".cyan(),
                    )
                );

                output.push_str(gnu_c);
            }
            Ok(TokenKind::GNUC_MINOR) => {
                // __GNUC_MINOR__
                // This macro is defined if and only if this is GNU C. This
                // macro is defined only when the entire GNU C compiler is in
                // use; if you invoke the preprocessor directly,
                // `__GNUC_MINOR__' is undefined. The value identifies the
                // minor version number of GNU CC.

                let gnu_c_minor = &macro_values
                    .iter()
                    .find(|m| m.macro_name == __GNUC_MINOR__)
                    .map(|m| m.value.clone())
                    .unwrap_or(DEFAULT_GNUC_MINOR.to_owned());

                tracing::debug!(
                    "{}",
                    &format!(
                        "  {}  {} {} {}{}{}",
                        "PREPROCESSOR".blue(),
                        __GNUC_MINOR__.cyan(),
                        "->".black(),
                        "'".cyan(),
                        gnu_c_minor.red(),
                        "'".cyan(),
                    )
                );

                output.push_str(gnu_c_minor);
            }
            Ok(TokenKind::BASEFILE) => {
                // __BASE_FILE__
                // This macro expands to the name of the main input file, in the form of a C
                // string constant. This is the source file that was specified on the command
                // line of the preprocessor or C compiler.

                tracing::debug!(
                    "{}",
                    &format!(
                        "  {}  {} {} {}{}{}",
                        "PREPROCESSOR".blue(),
                        __BASE_FILE__.cyan(),
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
            Ok(TokenKind::INCLUDE_LEVEL) => {
                //  __INCLUDE_LEVEL__
                // This macro expands to a decimal integer constant that represents the depth of
                // nesting in include files. The value of this macro is incremented on every
                // `#include' directive and decremented at every end of file. For input files
                // specified by command line arguments, the nesting level is zero.

                tracing::debug!(
                    "{}",
                    &format!(
                        "  {}   {} {} {}{}{}",
                        "PREPROCESSOR".blue(),
                        __INCLUDE_LEVEL__.cyan(),
                        "->".black(),
                        "'".cyan(),
                        include_level.to_string().red(),
                        "'".cyan(),
                    )
                );

                output.push('0');
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

fn process_line(line: &str, include_directories: &[String]) -> String {
    // Ignore empty lines
    if line.trim().is_empty() {
        return line.to_string();
    }

    tracing::trace!(
        "{}",
        &format!(
            "  {} Processing line {}{}{}",
            "PREPROCESSOR".blue(),
            "'".cyan(),
            line.yellow(),
            "'".cyan()
        )
    );

    // Check if the line starts with a preprocessing directive
    if line.trim().starts_with("#include") {
        // Parse the #include directive and retrieve the included file's name
        let included_file = parse_include_directive(line);

        tracing::trace!(
            "{}",
            &format!(
                "  {}  {} {} {}{}{}",
                "PREPROCESSOR".blue(),
                "#include".cyan(),
                "->".black(),
                "'".cyan(),
                included_file.yellow(),
                "'".cyan(),
            )
        );

        // Find and read the content of the included file
        // let include_content = read_included_file(&included_file, include_directories)
        //     .unwrap_or_else(|e| {
        //         tracing::error!("Error reading included file: {}", e);
        //         format!("/* {e} */")
        //     });

        // // Recursively process the included content (in case it has directives)
        // let processed_include_content = process_code(&include_content,
        // include_directories);

        // // Replace the #include directive with the processed content
        // return processed_include_content;

        return included_file; // temp
    }

    line.to_string()
}

fn parse_include_directive(line: &str) -> String {
    // Extract the filename from the #include directive

    let mut parts = line.split_whitespace();
    let include_directive = parts.next().unwrap_or("");
    let include_location = parts.next().unwrap_or("");
    println!("line: {line}");
    println!("include_directive: {include_directive}");
    println!("include_location: {include_location}");

    // "parsing include directive"
    tracing::debug!(
        "{}",
        &format!(
            "  {}  Parsing {} {} {} {}{}{}{}",
            "PREPROCESSOR".blue(),
            "#include".cyan(),
            "directive".green(),
            "->".black(),
            "'".cyan(),
            include_location.yellow(),
            "'".cyan(),
            "...".black(),
        )
    );

    if include_location.starts_with('<') && include_location.ends_with('>') {
        // Angle-bracket include
        tracing::trace!(
            "{}",
            &format!(
                "  {}  {} {} {}{}{}",
                "PREPROCESSOR".blue(),
                "#include".cyan(),
                "->".black(),
                "<".cyan(),
                include_location.yellow(),
                ">".cyan(),
            )
        );

        return include_location[1..include_location.len() - 1].to_string();
    } else if include_location.starts_with('"') && include_location.ends_with('"') {
        // Quoted include

        tracing::trace!(
            "{}",
            &format!(
                "  {}  {} {} {}{}{}",
                "PREPROCESSOR".blue(),
                "#include".cyan(),
                "->".black(),
                "\"".cyan(),
                include_location.yellow(),
                "\"".cyan(),
            )
        );

        return include_location[1..include_location.len() - 1].to_string();
    } else {
        // Invalid include
        println!("Invalid include 1: {line}");
        return format!("/* Invalid include: {line} */");
    }

    // println!("{:#?}", parts.nth(1));

    // todo!();

    // Check if there's at least one token after #include
    // if let Some(token) = include_directive {
    //     tracing::trace!(
    //         "{}",
    //         &format!(
    //             "  {}  {} {} {}{}{}",
    //             "PREPROCESSOR".blue(),
    //             "#include".cyan(),
    //             "->".black(),
    //             "'".cyan(),
    //             token.yellow(),
    //             "'".cyan(),
    //         )
    //     );

    //     println!("token: {token}");
    //     if token.starts_with('<') && token.ends_with('>') {
    //         // Angle-bracket include
    //         tracing::trace!(
    //             "{}",
    //             &format!(
    //                 "  {}  {} {} {}{}{}",
    //                 "PREPROCESSOR".blue(),
    //                 "#include".cyan(),
    //                 "->".black(),
    //                 "<".cyan(),
    //                 token.yellow(),
    //                 ">".cyan(),
    //             )
    //         );

    //         println!("token: {token}");

    //         token.to_string();
    //     } else if token.starts_with('"') && token.ends_with('"') {
    //         // Quoted include

    //         tracing::trace!(
    //             "{}",
    //             &format!(
    //                 "  {}  {} {} {}{}{}",
    //                 "PREPROCESSOR".blue(),
    //                 "#include".cyan(),
    //                 "->".black(),
    //                 "\"".cyan(),
    //                 token.yellow(),
    //                 "\"".cyan(),
    //             )
    //         );

    //         return token[1..token.len() - 1].to_string();
    //     } else {
    //         // Invalid include
    //         println!("Invalid include 1: {line}");
    //         return format!("/* Invalid include: {line} */");
    //     }
    // }

    format!("/* Invalid include: {line} */")
    //     if let Some(remainder) = parts.next() {
    //         if token.starts_with('<') && token.ends_with('>') {
    //             // Angle-bracket include
    //             tracing::trace!(
    //                 "{}",
    //                 &format!(
    //                     "  {}  {} {} {}{}{}",
    //                     "PREPROCESSOR".blue(),
    //                     "#include".cyan(),
    //                     "->".black(),
    //                     "<".cyan(),
    //                     token.yellow(),
    //                     ">".cyan(),
    //                 )
    //             );

    //             token.to_string()
    //         } else if token.starts_with('"') && token.ends_with('"') {
    //             // Quoted include

    //             tracing::trace!(
    //                 "{}",
    //                 &format!(
    //                     "  {}  {} {} {}{}{}",
    //                     "PREPROCESSOR".blue(),
    //                     "#include".cyan(),
    //                     "->".black(),
    //                     "\"".cyan(),
    //                     token.yellow(),
    //                     "\"".cyan(),
    //                 )
    //             );

    //             return token[1..token.len() - 1].to_string();
    //         } else {
    //             // Invalid include
    //             println!("Invalid include 1: {line}");
    //             format!("/* Invalid include: {line} */")
    //         }
    //     } else {
    //         // Invalid include
    //         println!("Invalid include 2: {line}");
    //         format!("/* Invalid include: {line} */")
    //     }
    // } else {
    //     // Invalid include
    //     println!("Invalid include 3: {line}");
    //     format!("/* Invalid include: {line} */")
    // }
}

// fn read_included_file(filename: &str, include_directories: &[String]) ->
// String {     // Implement logic to find and read the included file from the
// specified     // directories You'll need to search for the file in
// include_directories and     // read its content Return the content as a
// string     // Handle error cases (file not found, etc.) appropriately
//     // For simplicity, we'll assume the file content for this example
//     if filename == "stdio.h" {
//         // Simulate reading the content of stdio.h
//         return r#"
//             #pragma once

//             #include <stddef.h>

//             FILE *fopen(const char *filename, const char *mode);
//             int fclose(FILE *stream);
//             size_t fread(void *ptr, size_t size, size_t count, FILE *stream);
//             size_t fwrite(const void *ptr, size_t size, size_t count, FILE
// *stream);             int fprintf(FILE *stream, const char *format, ...);
//             int fscanf(FILE *stream, const char *format, ...);
//         "#
//         .to_string();
//     } else {
//         // Simulate reading the content of an unknown file
//         return format!("/* Contents of {} not found */", filename);
//     }
// }

fn read_included_file(filename: &str, include_directories: &[String]) -> Result<String, String> {
    // Iterate through each include directory to find the file
    for include_dir in include_directories {
        // Construct the full path to the file using the include directory
        let full_path = format!("{include_dir}/{filename}");

        tracing::trace!(
            "{}",
            &format!(
                "  {}  {} {}{}{}{}",
                "PREPROCESSOR".blue(),
                "Searching".black(),
                "'".cyan(),
                full_path.yellow(),
                "'".cyan(),
                "...".black(),
            )
        );

        // Attempt to open the file
        match File::open(&full_path) {
            Ok(mut file) => {
                // Read the content of the file into a string
                let mut content = String::new();
                if let Err(_) = file.read_to_string(&mut content) {
                    return Err(format!("Failed to read file: {}", full_path));
                }
                return Ok(content);
            }
            Err(_) => {
                // File not found in this directory, continue searching
                continue;
            }
        }
    }

    // If the file is not found in any include directory, return an error
    Err(format!("File not found: {}", filename))
}

fn process_code(code: &str, include_directories: &[String]) -> String {
    // Split the code into lines and process each line
    let lines: Vec<&str> = code.lines().collect();
    let processed_lines: Vec<String> =
        lines.iter().map(|line| process_line(line, include_directories)).collect();

    // Join the processed lines back into a single string
    processed_lines.join("\n")
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
    let include_directories =
        vec![String::from("/usr/include"), String::from("/usr/local/include"), String::from(".")];

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

                    preprocess_file(
                        input_path.to_str().unwrap(),
                        output_path.to_str().unwrap(),
                        &include_directories,
                    )?;

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
