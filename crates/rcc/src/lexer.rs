use std::{
    fmt::{self, Display},
    ops::Range,
};

use derive_more::Display;
use getset::{Getters, MutGetters, Setters};
// use getset::{Getters, MutGetters, Setters};
// use itertools::Itertools;
use logos::Logos;
use owo_colors::OwoColorize;
use smartstring::alias::String;
use typed_builder::TypedBuilder;

use crate::cst::SyntaxKind;

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Logos, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
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

    // #[token("    ")]
    // INDENT,
    // OUTDENT,
    UNKNOWN,
    #[end]
    EOF,
}
impl Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Operators
            TokenKind::PLUS => write!(f, "+"),
            TokenKind::MINUS => write!(f, "-"),
            TokenKind::STAR => write!(f, "*"),
            TokenKind::SLASH => write!(f, "/"),
            TokenKind::DSLASH => write!(f, "//"),
            TokenKind::PERCENT => write!(f, "%"),
            TokenKind::DSTAR => write!(f, "**"),
            TokenKind::TILDE => write!(f, "~"),
            TokenKind::AMP => write!(f, "&"),
            TokenKind::DOUBLEAMP => write!(f, "&&"),
            TokenKind::PIPE => write!(f, "|"),
            TokenKind::DOUBLEPIPE => write!(f, "||"),
            TokenKind::CARET => write!(f, "^"),
            TokenKind::LSHIFT => write!(f, "<<"),
            TokenKind::RSHIFT => write!(f, ">>"),
            TokenKind::EQ => write!(f, "="),
            TokenKind::LT => write!(f, "<"),
            TokenKind::GT => write!(f, ">"),
            TokenKind::GE => write!(f, ">="),
            TokenKind::LE => write!(f, "<="),
            TokenKind::EQEQ => write!(f, "=="),
            TokenKind::NE => write!(f, "!="),
            TokenKind::BANG => write!(f, "!"),
            TokenKind::PLUSEQ => write!(f, "+="),
            TokenKind::MINUSEQ => write!(f, "-="),
            TokenKind::STAREQ => write!(f, "*="),
            TokenKind::SLASHEQ => write!(f, "/="),
            TokenKind::DSLASHEQ => write!(f, "//="),
            TokenKind::PERCENTEQ => write!(f, "%="),
            TokenKind::AMPEQ => write!(f, "&="),
            TokenKind::PIPEEQ => write!(f, "|="),
            TokenKind::CARETEQ => write!(f, "^="),
            TokenKind::LSHIFTEQ => write!(f, "<<="),
            TokenKind::RSHIFTEQ => write!(f, ">>="),
            TokenKind::QUESTION => write!(f, "?"),
            TokenKind::PTR_OP => write!(f, "->"),
            TokenKind::INC_OP => write!(f, "++"),
            TokenKind::DEC_OP => write!(f, "--"),

            // Other Symbols
            TokenKind::DOT => write!(f, "."),
            TokenKind::COMMA => write!(f, ","),
            TokenKind::SEMICOLON => write!(f, ";"),
            TokenKind::COLON => write!(f, ":"),
            TokenKind::LPAREN => write!(f, "("),
            TokenKind::RPAREN => write!(f, ")"),
            TokenKind::LBRACKET => write!(f, "["),
            TokenKind::RBRACKET => write!(f, "]"),
            TokenKind::LBRACE => write!(f, "{{"),
            TokenKind::RBRACE => write!(f, "}}"),
            TokenKind::ELLIPSIS => write!(f, "..."),

            // Keywords
            TokenKind::AUTO_KW => write!(f, "auto"),
            TokenKind::BREAK_KW => write!(f, "break"),
            TokenKind::CASE_KW => write!(f, "case"),
            TokenKind::CHAR_KW => write!(f, "char"),
            TokenKind::CONST_KW => write!(f, "const"),
            TokenKind::CONTINUE_KW => write!(f, "continue"),
            TokenKind::DEFAULT_KW => write!(f, "default"),
            TokenKind::DO_KW => write!(f, "do"),
            TokenKind::DOUBLE_KW => write!(f, "double"),
            TokenKind::ELSE_KW => write!(f, "else"),
            TokenKind::ENUM_KW => write!(f, "enum"),
            TokenKind::EXTERN_KW => write!(f, "extern"),
            TokenKind::FLOAT_KW => write!(f, "float"),
            TokenKind::FOR_KW => write!(f, "for"),
            TokenKind::GOTO_KW => write!(f, "goto"),
            TokenKind::IF_KW => write!(f, "if"),
            TokenKind::INLINE_KW => write!(f, "inline"),
            TokenKind::INT_KW => write!(f, "int"),
            TokenKind::LONG_KW => write!(f, "long"),
            TokenKind::REGISTER_KW => write!(f, "register"),
            TokenKind::RESTRICT_KW => write!(f, "restrict"),
            TokenKind::RETURN_KW => write!(f, "return"),
            TokenKind::SHORT_KW => write!(f, "short"),
            TokenKind::SIGNED_KW => write!(f, "signed"),
            TokenKind::SIZEOF_KW => write!(f, "sizeof"),
            TokenKind::STATIC_KW => write!(f, "static"),
            TokenKind::STRUCT_KW => write!(f, "struct"),
            TokenKind::SWITCH_KW => write!(f, "switch"),
            TokenKind::TYPEDEF_KW => write!(f, "typedef"),
            TokenKind::UNION_KW => write!(f, "union"),
            TokenKind::UNSIGNED_KW => write!(f, "unsigned"),
            TokenKind::VOID_KW => write!(f, "void"),
            TokenKind::VOLATILE_KW => write!(f, "volatile"),
            TokenKind::WHILE_KW => write!(f, "while"),
            TokenKind::ALIGNAS_KW => write!(f, "_Alignas"),
            TokenKind::ALIGNOF_KW => write!(f, "_Alignof"),
            TokenKind::ATOMIC_KW => write!(f, "_Atomic"),
            TokenKind::BOOL_KW => write!(f, "_Bool"),
            TokenKind::COMPLEX_KW => write!(f, "_Complex"),
            TokenKind::GENERIC_KW => write!(f, "_Generic"),
            TokenKind::IMAGINARY_KW => write!(f, "_Imaginary"),
            TokenKind::NORETURN_KW => write!(f, "_Noreturn"),
            TokenKind::STATIC_ASSERT_KW => write!(f, "_Static_assert"),
            TokenKind::THREAD_LOCAL_KW => write!(f, "_Thread_local"),
            TokenKind::FUNC_NAME_KW => write!(f, "__func__"),

            // Identifier and Literals
            TokenKind::IDENTIFIER => write!(f, "IDENTIFIER"),
            TokenKind::STRING => write!(f, "string literal"),
            TokenKind::BYTES => write!(f, "bytes literal"),
            TokenKind::INTEGER_CONSTANT => write!(f, "integer constant"),
            TokenKind::FLOATING_CONSTANT => write!(f, "floating constant"),

            // Comments and Whitespace
            TokenKind::COMMENT => write!(f, "comment"),
            TokenKind::WHITESPACE => write!(f, "whitespace"),
            TokenKind::NEWLINE => write!(f, "newline"),

            // Unknown and EOF
            TokenKind::UNKNOWN => write!(f, "unknown"),
            TokenKind::EOF => write!(f, "end of file"),
        }
    }
}

impl TokenKind {
    /// Convert a given [`TokenKind`] to a [`SyntaxKind`].
    /// This is used to convert the tokens from the **lexer** to the tokens
    /// used in the **parser** and the **syntax tree**.
    pub const fn to_syntax(self) -> SyntaxKind {
        match self {
            TokenKind::PLUS => SyntaxKind::PLUS,
            TokenKind::MINUS => SyntaxKind::MINUS,
            TokenKind::STAR => SyntaxKind::STAR,
            TokenKind::SLASH => SyntaxKind::SLASH,
            TokenKind::DSLASH => SyntaxKind::DSLASH,
            TokenKind::PERCENT => SyntaxKind::PERCENT,
            TokenKind::DSTAR => SyntaxKind::DSTAR,
            TokenKind::TILDE => SyntaxKind::TILDE,
            TokenKind::AMP => SyntaxKind::AMP,
            TokenKind::DOUBLEAMP => SyntaxKind::DOUBLEAMP,
            TokenKind::PIPE => SyntaxKind::PIPE,
            TokenKind::DOUBLEPIPE => SyntaxKind::DOUBLEPIPE,
            TokenKind::CARET => SyntaxKind::CARET,
            TokenKind::LSHIFT => SyntaxKind::LSHIFT,
            TokenKind::RSHIFT => SyntaxKind::RSHIFT,
            TokenKind::EQ => SyntaxKind::EQ,
            TokenKind::LT => SyntaxKind::LT,
            TokenKind::GT => SyntaxKind::GT,
            TokenKind::GE => SyntaxKind::GE,
            TokenKind::LE => SyntaxKind::LE,
            TokenKind::EQEQ => SyntaxKind::EQEQ,
            TokenKind::NE => SyntaxKind::NE,
            TokenKind::BANG => SyntaxKind::BANG,
            TokenKind::PLUSEQ => SyntaxKind::PLUSEQ,
            TokenKind::MINUSEQ => SyntaxKind::MINUSEQ,
            TokenKind::STAREQ => SyntaxKind::STAREQ,
            TokenKind::SLASHEQ => SyntaxKind::SLASHEQ,
            TokenKind::DSLASHEQ => SyntaxKind::DSLASHEQ,
            TokenKind::PERCENTEQ => SyntaxKind::PERCENTEQ,
            TokenKind::AMPEQ => SyntaxKind::AMPEQ,
            TokenKind::PIPEEQ => SyntaxKind::PIPEEQ,
            TokenKind::CARETEQ => SyntaxKind::CARETEQ,
            TokenKind::LSHIFTEQ => SyntaxKind::LSHIFTEQ,
            TokenKind::RSHIFTEQ => SyntaxKind::RSHIFTEQ,
            TokenKind::QUESTION => SyntaxKind::QUESTION,
            TokenKind::PTR_OP => SyntaxKind::PTR_OP,
            TokenKind::INC_OP => SyntaxKind::INC_OP,
            TokenKind::DEC_OP => SyntaxKind::DEC_OP,
            TokenKind::DOT => SyntaxKind::DOT,
            TokenKind::COMMA => SyntaxKind::COMMA,
            TokenKind::SEMICOLON => SyntaxKind::SEMICOLON,
            TokenKind::COLON => SyntaxKind::COLON,
            TokenKind::LPAREN => SyntaxKind::LPAREN,
            TokenKind::RPAREN => SyntaxKind::RPAREN,
            TokenKind::LBRACKET => SyntaxKind::LBRACKET,
            TokenKind::RBRACKET => SyntaxKind::RBRACKET,
            TokenKind::LBRACE => SyntaxKind::LBRACE,
            TokenKind::RBRACE => SyntaxKind::RBRACE,
            TokenKind::ELLIPSIS => SyntaxKind::ELLIPSIS,
            TokenKind::AUTO_KW => SyntaxKind::AUTO_KW,
            TokenKind::BREAK_KW => SyntaxKind::BREAK_KW,
            TokenKind::CASE_KW => SyntaxKind::CASE_KW,
            TokenKind::CHAR_KW => SyntaxKind::CHAR_KW,
            TokenKind::CONST_KW => SyntaxKind::CONST_KW,
            TokenKind::CONTINUE_KW => SyntaxKind::CONTINUE_KW,
            TokenKind::DEFAULT_KW => SyntaxKind::DEFAULT_KW,
            TokenKind::DO_KW => SyntaxKind::DO_KW,
            TokenKind::DOUBLE_KW => SyntaxKind::DOUBLE_KW,
            TokenKind::ELSE_KW => SyntaxKind::ELSE_KW,
            TokenKind::ENUM_KW => SyntaxKind::ENUM_KW,
            TokenKind::EXTERN_KW => SyntaxKind::EXTERN_KW,
            TokenKind::FLOAT_KW => SyntaxKind::FLOAT_KW,
            TokenKind::FOR_KW => SyntaxKind::FOR_KW,
            TokenKind::GOTO_KW => SyntaxKind::GOTO_KW,
            TokenKind::IF_KW => SyntaxKind::IF_KW,
            TokenKind::INT_KW => SyntaxKind::INT_KW,
            TokenKind::LONG_KW => SyntaxKind::LONG_KW,
            TokenKind::REGISTER_KW => SyntaxKind::REGISTER_KW,
            TokenKind::RETURN_KW => SyntaxKind::RETURN_KW,
            TokenKind::SHORT_KW => SyntaxKind::SHORT_KW,
            TokenKind::SIGNED_KW => SyntaxKind::SIGNED_KW,
            TokenKind::SIZEOF_KW => SyntaxKind::SIZEOF_KW,
            TokenKind::STATIC_KW => SyntaxKind::STATIC_KW,
            TokenKind::STRUCT_KW => SyntaxKind::STRUCT_KW,
            TokenKind::SWITCH_KW => SyntaxKind::SWITCH_KW,
            TokenKind::TYPEDEF_KW => SyntaxKind::TYPEDEF_KW,
            TokenKind::UNION_KW => SyntaxKind::UNION_KW,
            TokenKind::UNSIGNED_KW => SyntaxKind::UNSIGNED_KW,
            TokenKind::VOID_KW => SyntaxKind::VOID_KW,
            TokenKind::VOLATILE_KW => SyntaxKind::VOLATILE_KW,
            TokenKind::IDENTIFIER => SyntaxKind::IDENTIFIER,
            // TokenKind::INT => SyntaxKind::INT,
            // TokenKind::FLOAT => SyntaxKind::FLOAT,
            TokenKind::STRING => SyntaxKind::STRING,
            TokenKind::BYTES => SyntaxKind::BYTES,
            // TokenKind::RUNE => SyntaxKind::RUNE,
            TokenKind::COMMENT => SyntaxKind::COMMENT,
            TokenKind::WHITESPACE => SyntaxKind::WHITESPACE,
            TokenKind::NEWLINE => SyntaxKind::NEWLINE,
            TokenKind::UNKNOWN => SyntaxKind::UNKNOWN,
            TokenKind::EOF => SyntaxKind::EOF,
            // TokenKind::CONSTANT => SyntaxKind::CONSTANT,
            TokenKind::WHILE_KW => SyntaxKind::WHILE_KW,
            TokenKind::INLINE_KW => SyntaxKind::INLINE_KW,
            TokenKind::RESTRICT_KW => SyntaxKind::RESTRICT_KW,
            TokenKind::ALIGNAS_KW => SyntaxKind::ALIGNAS_KW,
            TokenKind::ALIGNOF_KW => SyntaxKind::ALIGNOF_KW,
            TokenKind::ATOMIC_KW => SyntaxKind::ATOMIC_KW,
            TokenKind::BOOL_KW => SyntaxKind::BOOL_KW,
            TokenKind::COMPLEX_KW => SyntaxKind::COMPLEX_KW,
            TokenKind::GENERIC_KW => SyntaxKind::GENERIC_KW,
            TokenKind::IMAGINARY_KW => SyntaxKind::IMAGINARY_KW,
            TokenKind::NORETURN_KW => SyntaxKind::NORETURN_KW,
            TokenKind::STATIC_ASSERT_KW => SyntaxKind::STATIC_ASSERT_KW,
            TokenKind::THREAD_LOCAL_KW => SyntaxKind::THREAD_LOCAL_KW,
            TokenKind::INTEGER_CONSTANT => SyntaxKind::INTEGER_CONSTANT,
            TokenKind::FLOATING_CONSTANT => SyntaxKind::FLOATING_CONSTANT,
            TokenKind::FUNC_NAME_KW => SyntaxKind::FUNC_NAME_KW,
        }
    }

    pub(crate) fn is_declaration_specifier(&self) -> bool {
        self.is_storage_class_specifier()
            || self.is_type_specifier()
            || self.is_type_qualifier()
            || self.is_function_specifier()
            || self.is_alignment_specifier()
    }

    pub(crate) fn is_semicolon(&self) -> bool {
        matches!(self, TokenKind::SEMICOLON)
    }

    pub(crate) fn is_declarator(&self) -> bool {
        self.is_direct_declarator() || self.is_pointer()
    }

    pub(crate) fn is_direct_declarator(&self) -> bool {
        matches!(self, TokenKind::IDENTIFIER | TokenKind::LPAREN)
    }

    pub(crate) fn is_pointer(&self) -> bool {
        matches!(self, TokenKind::STAR)
    }

    pub(crate) fn is_equal(&self) -> bool {
        matches!(self, TokenKind::EQ)
    }

    pub(crate) fn is_storage_class_specifier(&self) -> bool {
        matches!(
            self,
            TokenKind::AUTO_KW
                | TokenKind::EXTERN_KW
                | TokenKind::REGISTER_KW
                | TokenKind::STATIC_KW
                | TokenKind::THREAD_LOCAL_KW
        )
    }

    pub(crate) fn is_type_specifier(&self) -> bool {
        matches!(
            self,
            TokenKind::VOID_KW
                | TokenKind::CHAR_KW
                | TokenKind::SHORT_KW
                | TokenKind::INT_KW
                | TokenKind::LONG_KW
                | TokenKind::FLOAT_KW
                | TokenKind::DOUBLE_KW
                | TokenKind::SIGNED_KW
                | TokenKind::UNSIGNED_KW
                | TokenKind::BOOL_KW
                | TokenKind::COMPLEX_KW
                | TokenKind::IMAGINARY_KW
                | TokenKind::STRUCT_KW
                | TokenKind::UNION_KW
                | TokenKind::ENUM_KW
                | TokenKind::TYPEDEF_KW
                | TokenKind::INLINE_KW
                | TokenKind::RESTRICT_KW
                | TokenKind::ATOMIC_KW
                | TokenKind::GENERIC_KW
                | TokenKind::NORETURN_KW
                | TokenKind::STATIC_ASSERT_KW
        )
    }

    pub(crate) fn is_function_specifier(&self) -> bool {
        matches!(self, TokenKind::INLINE_KW | TokenKind::NORETURN_KW)
    }

    pub(crate) fn is_type_qualifier(&self) -> bool {
        matches!(
            self,
            TokenKind::CONST_KW
                | TokenKind::RESTRICT_KW
                | TokenKind::VOLATILE_KW
                | TokenKind::ATOMIC_KW
        )
    }

    pub(crate) fn is_alignment_specifier(&self) -> bool {
        matches!(self, TokenKind::ALIGNAS_KW)
    }

    pub(crate) fn is_l_brace(&self) -> bool {
        matches!(self, TokenKind::LBRACE)
    }
}

#[derive(
    Debug,
    Display,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Ord,
    PartialOrd,
    Getters,
    MutGetters,
    Setters,
    TypedBuilder,
)]
#[display(fmt = "{start}..{end}")]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct Span {
    start: usize,
    end: usize,
}

// use rowan::{TextRange, TextSize};

// impl From<Span> for TextRange {
//   fn from(val: Span) -> Self {
//     TextRange::new(
//       TextSize::from(val.start as u32),
//       TextSize::from(val.end as u32),
//     )
//   }
// }

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Self { start: range.start, end: range.end }
    }
}

impl Span {
    pub fn merge(&self, range: Range<usize>) -> Self {
        Self { start: self.start.min(range.start), end: self.end.max(range.end) }
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Getters, MutGetters, Setters, TypedBuilder,
)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, span: Span) -> Self {
        Self { kind, lexeme, span }
    }

    pub fn pretty_print(&self) -> String {
        format!("{} {} {}", self.kind.blue(), self.lexeme, self.span.black().italic(),).into()
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lexeme)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters, MutGetters, TypedBuilder)]
#[getset(set = "pub", get_mut = "pub")]
pub struct TokenSink {
    pub tokens: TokenStream,
    // pub lexical_errors: Vec<Diagnostic<FileId>>,
}

impl TokenSink {
    pub fn new(input: &str) -> Self {
        Self {
            tokens: TokenStream::new(input),
            // lexical_errors: Vec::new(),
        }
    }

    pub fn pretty_print(&self) -> String {
        self.tokens.pretty_print()
    }

    pub fn tokens(&self) -> &[Token] {
        &self.tokens.tokens
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Getters, MutGetters, Setters, TypedBuilder,
)]
pub struct TokenStream {
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    text: String,
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    tokens: Vec<Token>,
    #[builder(default = 0)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    cursor: usize,
    // #[getset(get = "pub", get_mut = "pub", set = "pub")]
    // file_name: PathBuf,
    // file_id: FileId,
}

impl TokenStream {
    pub fn new(input: &str) -> Self {
        Self {
            text: String::from(input),
            tokens: Vec::new(),
            cursor: 0,
            // file_name: PathBuf::new(),
            // file_id: FileId::default(),
        }
    }

    pub fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn get(&self, index: usize) -> Option<Token> {
        self.tokens.get(index).cloned()
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn pretty_print(&self) -> String {
        self.tokens
            .iter()
            .map(|token| token.pretty_print())
            .collect::<Vec<String>>()
            .join("\n")
            .into()
    }
}

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.tokens.get(self.cursor).cloned();
        self.cursor += 1;
        token
    }
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
                if let Some(unknown_token) = current_unknown_token.clone() {
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
                    " {}  Creating token {:?} at {:?}",
                    "LEXER".green(),
                    token.yellow(),
                    lexer.span().black().italic()
                );

                // If the token is a double star, we want to convert it to two single stars and
                // add them to the token sink.

                if token == TokenKind::DSTAR {
                    tracing::trace!(
                        " {}  Creating token {:?} at {:?}",
                        "LEXER".green(),
                        TokenKind::STAR.yellow(),
                        lexer.span().black().italic()
                    );

                    token_sink.tokens.push(Token::new(
                        TokenKind::STAR,
                        "*".to_string().into(),
                        (lexer.span().start..lexer.span().start + 1).into(),
                    ));

                    token_sink.tokens.push(Token::new(
                        TokenKind::STAR,
                        "*".to_string().into(),
                        (lexer.span().start + 1..lexer.span().end).into(),
                    ));

                    continue;
                }

                token_sink.tokens.push(Token::new(
                    token,
                    lexer.slice().to_string().into(),
                    lexer.span().into(),
                ));
            }
            Err(()) => {
                if let Some(unknown_token) = current_unknown_token.clone() {
                    let Token { kind: _, span, lexeme } = unknown_token;

                    let span = span.merge(lexer.span());
                    let updated_lexeme = format!("{}{}", lexeme, lexer.slice());

                    tracing::debug!(
                        "Gluing together unknown tokens {} and {} to form {} at {}",
                        lexeme,
                        lexer.slice(),
                        updated_lexeme,
                        span
                    );

                    current_unknown_token =
                        Some(Token::new(TokenKind::UNKNOWN, updated_lexeme.into(), span));
                } else {
                    tracing::trace!(
                        " {}  Creating unknown token {:?} at {:?}",
                        "LEXER".green(),
                        TokenKind::UNKNOWN.yellow(),
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

    token_sink.tokens.push(Token::new(TokenKind::EOF, "".to_string().into(), lexer.span().into()));

    // Collect token information and format it as a tree
    let mut token_info = vec![];
    for token in token_sink.tokens().iter() {
        token_info.push(token);
    }
    let formatted_tokens = format_tokens_as_tree(&token_info, "  ");

    tracing::debug!("\n\n{}{}\n\n{}", "Token Stream".blue(), ":".black(), formatted_tokens);

    let elapsed = start.elapsed();

    tracing::info!(
        " {}   {} {} constructed{}{}{}",
        "LEXER".yellow(),
        " SUCCESS ".black().on_green(),
        "Token Stream".blue(),
        " in ".black(),
        format!("{elapsed:?}").cyan(),
        ".".black()
    );

    token_sink
}

fn format_tokens_as_tree(tokens: &[&Token], indent: &str) -> String {
    let mut formatted = String::new();

    for (i, token) in tokens.iter().enumerate() {
        formatted.push_str(&format!(
            "{indent}{}{:?}{}{}\n",
            "└─".black(),
            token.kind.green(),
            "@".yellow(),
            token.span.black().italic()
        ));

        formatted.push_str(&format!(
            "{indent}  {} {:?}@{} {}{}{}\n",
            "\\-".magenta(),
            token.kind.blue(),
            token.span.black().italic(),
            "'".red(),
            token.lexeme,
            "'".red(),
        ));

        if i < tokens.len() - 1 {
            formatted.push('\n');
        }
    }

    formatted
}
