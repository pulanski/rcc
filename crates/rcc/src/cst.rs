#![allow(bad_style, missing_docs, unreachable_pub)]
use num_derive::{FromPrimitive, ToPrimitive};
use strum_macros::EnumCount;

#[allow(clippy::manual_non_exhaustive, non_snake_case, non_camel_case_types)]
#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, FromPrimitive, ToPrimitive, EnumCount,
)]
#[repr(u16)]
pub enum SyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc(hidden)]
    EOF,
    ///Literals (e.g. IDENTIFIER, INT, FLOAT, STRING, BYTES)
    IDENTIFIER,
    INT,
    FLOAT,
    STRING,
    BYTES,
    ///Tokens (e.g. WHITESPACE, COMMENT, NEWLINE)
    WHITESPACE,
    COMMENT,
    NEWLINE,
    INDENT,
    OUTDENT,
    UNKNOWN,
    CONSTANT,
    ///Keywords (e.g. BREAK, IN, LET, LOOP, etc.)
    AND_KW,
    ELLIPSIS,
    AUTO_KW,
    CASE_KW,
    CHAR_KW,
    CONST_KW,
    DEFAULT_KW,
    DO_KW,
    DOUBLE_KW,
    ENUM_KW,
    EXTERN_KW,
    ELSE_KW,
    FLOAT_KW,
    LOAD_KW,
    BREAK_KW,
    FOR_KW,
    NOT_KW,
    GOTO_KW,
    RUNE,
    CONTINUE_KW,
    IF_KW,
    INT_KW,
    LONG_KW,
    OR_KW,
    DEF_KW,
    IN_KW,
    PASS_KW,
    ELIF_KW,
    REGISTER_KW,
    LAMBDA_KW,
    SHORT_KW,
    SIGNED_KW,
    SIZEOF_KW,
    STATIC_KW,
    STRUCT_KW,
    SWITCH_KW,
    TYPEDEF_KW,
    UNION_KW,
    UNSIGNED_KW,
    VOID_KW,
    RETURN_KW,
    VOLATILE_KW,
    WHILE_KW,
    ///Punctuation (e.g. DOT, COMMA, SEMICOLON, etc.)
    PLUS,
    MINUS,
    STAR,
    SLASH,
    DSLASH,
    PERCENT,
    DSTAR,
    TILDE,
    AMP,
    DOUBLEAMP,
    PIPE,
    BANG,
    DOUBLEPIPE,
    CARET,
    LSHIFT,
    RSHIFT,
    DOT,
    COMMA,
    EQ,
    SEMICOLON,
    COLON,
    LPAREN,
    RPAREN,
    LBRACKET,
    RBRACKET,
    LBRACE,
    RBRACE,
    LT,
    GT,
    GE,
    LE,
    EQEQ,
    NE,
    PLUSEQ,
    MINUSEQ,
    STAREQ,
    SLASHEQ,
    PERCENTEQ,
    AMPEQ,
    PIPEEQ,
    CARETEQ,
    LSHIFTEQ,
    RSHIFTEQ,
    DEC_OP,
    INC_OP,
    PTR_OP,
    QUESTION,
    DSLASHEQ,
    ///Nodes (e.g. FILE, MODULE, FUNCTION, etc.)
    FILE,
    STATEMENT,
    DEF_STMT,
    PARAMETERS,
    PARAMETER,
    IF_STMT,
    ELIF_CLAUSES,
    ELSE_CLAUSE,
    FOR_STMT,
    SUITE,
    SIMPLE_STMT,
    SMALL_STMT,
    RETURN_STMT,
    BREAK_STMT,
    CONTINUE_STMT,
    PASS_STMT,
    ASSIGN_STMT,
    EXPR_STMT,
    LOAD_STMT,
    TEST,
    BIN_OP,
    IF_EXPR,
    PRIMARY_EXPR,
    OPERAND,
    DOT_SUFFIX,
    SLICE_SUFFIX,
    CALL_SUFFIX,
    ARGUMENTS,
    ARGUMENT,
    LIST_EXPR,
    LIST_COMP,
    DICT_EXPR,
    DICT_COMP,
    ENTRIES,
    ENTRY,
    COMP_CLAUSE,
    UNARY_EXPR,
    BINARY_EXPR,
    BINOP,
    LAMBDA_EXPR,
    EXPRESSION,
    LOOP_VARIABLES,
    #[doc(hidden)]
    __LAST,
    INLINE_KW,
    RESTRICT_KW,
    ALIGNAS_KW,
    ALIGNOF_KW,
    ATOMIC_KW,
    BOOL_KW,
    COMPLEX_KW,
    GENERIC_KW,
    IMAGINARY_KW,
    NORETURN_KW,
    STATIC_ASSERT_KW,
    THREAD_LOCAL_KW,
    INTEGER_CONSTANT,
    FLOATING_CONSTANT,
    FUNC_NAME_KW,
}
use self::SyntaxKind::*;
impl SyntaxKind {
    pub fn is_keyword(self) -> bool {
        matches!(
            self,
            AND_KW
                | ELSE_KW
                | LOAD_KW
                | BREAK_KW
                | FOR_KW
                | NOT_KW
                | CONTINUE_KW
                | IF_KW
                | OR_KW
                | DEF_KW
                | IN_KW
                | PASS_KW
                | ELIF_KW
                | LAMBDA_KW
                | RETURN_KW
        )
    }
    pub fn is_punct(self) -> bool {
        matches!(
            self,
            PLUS | MINUS
                | STAR
                | SLASH
                | DSLASH
                | PERCENT
                | DSTAR
                | TILDE
                | AMP
                | PIPE
                | CARET
                | LSHIFT
                | RSHIFT
                | DOT
                | COMMA
                | EQ
                | SEMICOLON
                | COLON
                | LPAREN
                | RPAREN
                | LBRACKET
                | RBRACKET
                | LBRACE
                | RBRACE
                | LT
                | GT
                | GE
                | LE
                | EQEQ
                | NE
                | PLUSEQ
                | MINUSEQ
                | STAREQ
                | SLASHEQ
                | PERCENTEQ
                | AMPEQ
                | PIPEEQ
                | CARETEQ
                | LSHIFTEQ
                | RSHIFTEQ
        )
    }
    pub fn is_literal(self) -> bool {
        matches!(self, IDENTIFIER | INT | FLOAT | STRING | BYTES)
    }
    pub fn from_keyword(ident: &str) -> Option<SyntaxKind> {
        let kw = match ident {
            "and" => AND_KW,
            "else" => ELSE_KW,
            "load" => LOAD_KW,
            "break" => BREAK_KW,
            "for" => FOR_KW,
            "not" => NOT_KW,
            "continue" => CONTINUE_KW,
            "if" => IF_KW,
            "or" => OR_KW,
            "def" => DEF_KW,
            "in" => IN_KW,
            "pass" => PASS_KW,
            "elif" => ELIF_KW,
            "lambda" => LAMBDA_KW,
            "return" => RETURN_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub fn from_contextual_keyword(ident: &str) -> Option<SyntaxKind> {
        let kw = match ident {
            _ => return None,
        };
        Some(kw)
    }
    pub fn from_char(c: char) -> Option<SyntaxKind> {
        let tok = match c {
            '+' => PLUS,
            '-' => MINUS,
            '*' => STAR,
            '/' => SLASH,
            '%' => PERCENT,
            '~' => TILDE,
            '&' => AMP,
            '|' => PIPE,
            '^' => CARET,
            '.' => DOT,
            ',' => COMMA,
            '=' => EQ,
            ';' => SEMICOLON,
            ':' => COLON,
            '(' => LPAREN,
            ')' => RPAREN,
            '[' => LBRACKET,
            ']' => RBRACKET,
            '{' => LBRACE,
            '}' => RBRACE,
            '<' => LT,
            '>' => GT,
            _ => return None,
        };
        Some(tok)
    }
}
