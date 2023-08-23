use std::{
    cell::Cell,
    fmt::{self, Display},
};

use owo_colors::OwoColorize;
use smartstring::alias::String;

use tracing_subscriber::fmt::Subscriber;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum TokenKind {
    ErrorToken,
    Eof,

    LParen,
    RParen,
    LCurly,
    RCurly,
    Eq,
    Semi,
    Comma,
    Colon,
    Arrow,
    Plus,
    Minus,
    Star,
    Slash,

    FnKeyword,
    LetKeyword,
    ReturnKeyword,
    TrueKeyword,
    FalseKeyword,

    Name,
    Int,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TokenKind::ErrorToken => "an error",
            TokenKind::Eof => "end of file",
            TokenKind::LParen => "'('",
            TokenKind::RParen => "')'",
            TokenKind::LCurly => "'{'",
            TokenKind::RCurly => "'}'",
            TokenKind::Eq => "'='",
            TokenKind::Semi => "';'",
            TokenKind::Comma => "','",
            TokenKind::Colon => "':'",
            TokenKind::Arrow => "'->'",
            TokenKind::Plus => "'+'",
            TokenKind::Minus => "'-'",
            TokenKind::Star => "'*'",
            TokenKind::Slash => "'/'",
            TokenKind::FnKeyword => "'fn'",
            TokenKind::LetKeyword => "'let'",
            TokenKind::ReturnKeyword => "'return'",
            TokenKind::TrueKeyword => "'true'",
            TokenKind::FalseKeyword => "'false'",
            TokenKind::Name => "a name",
            TokenKind::Int => "an integer",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug)]
enum TreeKind {
    ErrorTree,
    File,
    Fn,
    TypeExpr,
    ParamList,
    Param,
    Block,
    StmtLet,
    StmtReturn,
    StmtExpr,
    ExprLiteral,
    ExprName,
    ExprParen,
    ExprBinary,
    ExprCall,
    ArgList,
    Arg,
}

#[macro_export]
macro_rules! format_to {
    ($buf:expr) => ();
    ($buf:expr, $lit:literal $($arg:tt)*) => {
        { use ::std::fmt::Write as _; let _ = ::std::write!($buf, $lit $($arg)*); }
    };
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    text: String,
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} '{}'", self.kind, self.text)
    }
}

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

pub struct Tree {
    kind: TreeKind,
    children: Vec<Child>,
}

impl Tree {
    fn print(&self, buf: &mut String, level: usize) {
        let indent = "  ".repeat(level);
        format_to!(buf, "{indent}{:?}\n", self.kind);
        for child in &self.children {
            match child {
                Child::Token(token) => {
                    format_to!(buf, "{indent}  '{}'\n", token.text)
                }
                Child::Tree(tree) => tree.print(buf, level + 1),
            }
        }
        assert!(buf.ends_with('\n'));
    }
}

enum Child {
    Token(Token),
    Tree(Tree),
}

impl fmt::Debug for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        self.print(&mut buf, 0);
        write!(f, "{buf}")
    }
}

fn lex(mut text: &str) -> Vec<Token> {
    let punctuation = (
        "( ) { } = ; , : -> + - * /",
        [
            TokenKind::LParen,
            TokenKind::RParen,
            TokenKind::LCurly,
            TokenKind::RCurly,
            TokenKind::Eq,
            TokenKind::Semi,
            TokenKind::Comma,
            TokenKind::Colon,
            TokenKind::Arrow,
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Star,
            TokenKind::Slash,
        ],
    );

    let keywords = (
        "fn let return true false",
        [
            TokenKind::FnKeyword,
            TokenKind::LetKeyword,
            TokenKind::ReturnKeyword,
            TokenKind::TrueKeyword,
            TokenKind::FalseKeyword,
        ],
    );

    let mut result = Vec::new();
    while !text.is_empty() {
        if let Some(rest) = trim(text, |it| it.is_ascii_whitespace()) {
            text = rest;
            continue;
        }
        let text_orig = text;
        let mut kind = 'kind: {
            for (i, symbol) in punctuation.0.split_ascii_whitespace().enumerate() {
                if let Some(rest) = text.strip_prefix(symbol) {
                    text = rest;
                    break 'kind punctuation.1[i];
                }
            }
            if let Some(rest) = trim(text, |it| it.is_ascii_digit()) {
                text = rest;
                break 'kind TokenKind::Int;
            }
            if let Some(rest) = trim(text, name_char) {
                text = rest;
                break 'kind TokenKind::Name;
            }
            let error_index = text.find(|it: char| it.is_ascii_whitespace()).unwrap_or(text.len());
            text = &text[error_index..];
            TokenKind::ErrorToken
        };
        assert!(text.len() < text_orig.len());
        let token_text = &text_orig[..text_orig.len() - text.len()];
        if kind == TokenKind::Name {
            for (i, symbol) in keywords.0.split_ascii_whitespace().enumerate() {
                if token_text == symbol {
                    kind = keywords.1[i];
                    break;
                }
            }
        }
        result.push(Token { kind, text: token_text.to_string().into() })
    }
    return result;

    fn name_char(c: char) -> bool {
        matches!(c, '_' | 'a'..='z' | 'A'..='Z' | '0'..='9')
    }

    fn trim(text: &str, predicate: impl std::ops::Fn(char) -> bool) -> Option<&str> {
        let index = text.find(|it: char| !predicate(it)).unwrap_or(text.len());
        if index == 0 {
            None
        } else {
            Some(&text[index..])
        }
    }
}

#[derive(Debug)]
enum Event {
    Open { kind: TreeKind },
    Close,
    Advance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MarkOpened {
    index: usize,
}

impl MarkOpened {
    fn new(index: usize) -> MarkOpened {
        MarkOpened { index }
    }
}

struct MarkClosed {
    index: usize,
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    fuel: Cell<u32>,
    events: Vec<Event>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, pos: 0, fuel: Cell::new(256), events: Vec::new() }
    }

    fn build_tree(self) -> Tree {
        let mut tokens = self.tokens.into_iter();
        let mut events = self.events;
        let mut stack = Vec::new();

        // Special case: pop the last `Close` event to ensure
        // that the stack is non-empty inside the loop.
        assert!(matches!(events.pop(), Some(Event::Close)));

        for event in events {
            match event {
                // Starting a new node; just push an empty tree to the stack.
                Event::Open { kind } => stack.push(Tree { kind, children: Vec::new() }),

                // A tree is done.
                // Pop it off the stack and append to a new current tree.
                Event::Close => {
                    let tree = stack.pop().unwrap();
                    stack.last_mut().unwrap().children.push(Child::Tree(tree));
                }

                // Consume a token and append it to the current tree.
                Event::Advance => {
                    let token = tokens.next().unwrap();
                    stack.last_mut().unwrap().children.push(Child::Token(token));
                }
            }
        }

        let tree = stack.pop().unwrap();

        // Our parser will guarantee that all the trees are closed
        // and cover the entirety of tokens.
        assert!(stack.is_empty());
        assert!(tokens.next().is_none());
        tree
    }

    fn open(&mut self) -> MarkOpened {
        let mark = MarkOpened { index: self.events.len() };
        self.events.push(Event::Open { kind: TreeKind::ErrorTree });
        mark
    }

    fn open_before(&mut self, m: MarkClosed) -> MarkOpened {
        let mark = MarkOpened { index: m.index };
        self.events.insert(m.index, Event::Open { kind: TreeKind::ErrorTree });
        mark
    }

    fn close(&mut self, m: MarkOpened, kind: TreeKind) -> MarkClosed {
        self.events[m.index] = Event::Open { kind };
        self.events.push(Event::Close);
        MarkClosed { index: m.index }
    }

    fn advance(&mut self) {
        assert!(!self.eof());
        self.fuel.set(256);
        self.events.push(Event::Advance);
        self.pos += 1;
    }

    fn advance_with_error(&mut self, error: &str) {
        let m = self.open();
        // TODO: Error reporting.
        tracing::error!("{error}");
        self.advance();
        self.close(m, TreeKind::ErrorTree);
    }

    fn eof(&self) -> bool {
        self.pos == self.tokens.len()
    }

    fn nth(&self, lookahead: usize) -> TokenKind {
        if self.fuel.get() == 0 {
            panic!("parser is stuck")
        }
        self.fuel.set(self.fuel.get() - 1);
        self.tokens.get(self.pos + lookahead).map_or(TokenKind::Eof, |it| it.kind)
    }

    fn at(&self, kind: TokenKind) -> bool {
        self.nth(0) == kind
    }

    fn at_any(&self, kinds: &[TokenKind]) -> bool {
        kinds.contains(&self.nth(0))
    }

    fn eat(&mut self, kind: TokenKind) -> bool {
        if self.at(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, kind: TokenKind) {
        let found = self.nth(0);
        if self.eat(kind) {
            return;
        }
        // TODO: Error reporting.

        tracing::error!(
            "Expected {expected}{comma} but instead found {found}{period}",
            expected = kind.yellow(),
            comma = ",".black(),
            found = found.red(),
            period = ".".black(),
        );
    }
}

// File = Fn*
fn file(p: &mut Parser) {
    tracing::debug!(
        "{}",
        &format!("{} {:?} {} {}", "PARSER".yellow(), p.nth(0), "->".yellow(), "file".green())
    );
    let m = p.open();

    let mut error_emitted = false; // Initialize the error flag.

    while !p.eof() {
        if p.at(TokenKind::FnKeyword) {
            func(p);
            error_emitted = false; // Reset the error flag when "fn" is found.
        } else if !error_emitted {
            let error_tree = p.open(); // Start a new error tree.

            tracing::error!(
                "Expected {expected}{comma} but instead found {found}{period}",
                expected = "'fn'".yellow(),
                comma = ",".black(),
                found = p.nth(0).red(),
                period = ".".black(),
            );

            while !p.at(TokenKind::FnKeyword) && !p.eof() {
                p.advance(); // Consume tokens until "fn" is found.
            }

            p.close(error_tree, TreeKind::ErrorTree); // Close the error tree.

            error_emitted = true; // Set the error flag to true after emitting the error.
        } else {
            // Consume the non-"fn" token without emitting additional errors.
            p.advance();
        }
    }

    p.close(m, TreeKind::File);
}

// Fn = 'fn' 'name' ParamList ('->' TypeExpr)? Block
fn func(p: &mut Parser) {
    tracing::debug!(
        "{}",
        &format!("{} {:?} {} {}", "PARSER".yellow(), p.nth(0), "->".yellow(), "func".green())
    );
    assert!(p.at(TokenKind::FnKeyword));
    let m = p.open();

    p.expect(TokenKind::FnKeyword);
    p.expect(TokenKind::Name);
    if p.at(TokenKind::LParen) {
        param_list(p);
    }
    if p.eat(TokenKind::Arrow) {
        type_expr(p);
    }
    if p.at(TokenKind::LCurly) {
        block(p);
    }
    p.close(m, TreeKind::Fn);
}

// ParamList = '(' Param* ')'
fn param_list(p: &mut Parser) {
    tracing::debug!(
        "{}",
        &format!("{} {:?} {} {}", "PARSER".yellow(), p.nth(0), "->".yellow(), "param_list".green())
    );
    assert!(p.at(TokenKind::LParen));
    let m = p.open();

    p.expect(TokenKind::LParen);
    while !p.at(TokenKind::RParen) && !p.eof() {
        if p.at(TokenKind::Name) {
            param(p);
        } else {
            break;
        }
    }
    p.expect(TokenKind::RParen);
    p.close(m, TreeKind::ParamList);
}
const STMT_RECOVERY: &[TokenKind] = &[TokenKind::FnKeyword];
const EXPR_FIRST: &[TokenKind] = &[
    TokenKind::Int,
    TokenKind::TrueKeyword,
    TokenKind::FalseKeyword,
    TokenKind::Name,
    TokenKind::LParen,
];
const STMT_EXPECTED: &[TokenKind] = &[
    TokenKind::LetKeyword,
    TokenKind::ReturnKeyword,
    TokenKind::LCurly,
    TokenKind::FnKeyword,
    TokenKind::Name,
    TokenKind::Int,
    TokenKind::TrueKeyword,
    TokenKind::FalseKeyword,
    TokenKind::LParen,
];
// Block = '{' Stmt* '}'
//
// Stmt =
//   StmtLet
// | StmtReturn
// | StmtExpr
fn block(p: &mut Parser) {
    tracing::debug!(
        "{}",
        &format!("{} {:?} {} {}", "PARSER".yellow(), p.nth(0), "->".yellow(), "block".green())
    );
    assert!(p.at(TokenKind::LCurly));
    let m = p.open();

    p.expect(TokenKind::LCurly);
    while !p.at(TokenKind::RCurly) && !p.eof() {
        match p.nth(0) {
            TokenKind::LetKeyword => stmt_let(p),
            TokenKind::ReturnKeyword => stmt_return(p),
            _ => {
                if p.at_any(EXPR_FIRST) {
                    stmt_expr(p)
                } else {
                    if p.at_any(STMT_RECOVERY) {
                        break;
                    }

                    let expected = STMT_EXPECTED
                        .iter()
                        .map(|k| format!("{k}", k = k.yellow()))
                        .collect::<Vec<_>>()
                        .join(format!("{comma} ", comma = ",".black()).as_str());

                    let found: String =
                        format!("{found}", found = p.nth(0)).red().to_string().into();

                    p.advance_with_error(&format!(
                        "Expected one of {expected}{comma} but instead found {found}{period}",
                        comma = ",".black(),
                        period = ".".black()
                    ));
                }
            }
        }
    }
    p.expect(TokenKind::RCurly);

    p.close(m, TreeKind::Block);
}

// StmtLet = 'let' 'name' '=' Expr ';'
fn stmt_let(p: &mut Parser) {
    tracing::debug!(
        "{}",
        &format!("{} {:?} {} {}", "PARSER".yellow(), p.nth(0), "->".yellow(), "stmt_let".green())
    );

    assert!(p.at(TokenKind::LetKeyword));
    let m = p.open();

    p.expect(TokenKind::LetKeyword);
    p.expect(TokenKind::Name);
    p.expect(TokenKind::Eq);
    expr(p);
    p.expect(TokenKind::Semi);
    p.close(m, TreeKind::StmtLet);
}
// StmtReturn = 'return' Expr ';'
fn stmt_return(p: &mut Parser) {
    tracing::debug!(
        "{}",
        &format!(
            "{} {:?} {} {}",
            "PARSER".yellow(),
            p.nth(0),
            "->".yellow(),
            "stmt_return".green()
        )
    );

    assert!(p.at(TokenKind::ReturnKeyword));
    let m = p.open();
    p.expect(TokenKind::ReturnKeyword);
    expr(p);
    p.expect(TokenKind::Semi);
    p.close(m, TreeKind::StmtReturn);
}
// StmtExpr = Expr ';'
fn stmt_expr(p: &mut Parser) {
    tracing::debug!(
        "{}",
        &format!("{} {:?} {} {}", "PARSER".yellow(), p.nth(0), "->".yellow(), "stmt_expr".green())
    );

    let m = p.open();
    expr(p);
    p.expect(TokenKind::Semi);
    p.close(m, TreeKind::StmtExpr);
}

fn expr(p: &mut Parser) {
    tracing::debug!(
        "{}",
        &format!("{} {:?} {} {}", "PARSER".yellow(), p.nth(0), "->".yellow(), "expr".green())
    );

    expr_rec(p, TokenKind::Eof);
}
fn expr_rec(p: &mut Parser, left: TokenKind) {
    let Some(mut lhs) = expr_delimited(p) else {
    return;
  };

    while p.at(TokenKind::LParen) {
        let m = p.open_before(lhs);
        arg_list(p);
        lhs = p.close(m, TreeKind::ExprCall);
    }
    loop {
        let right = p.nth(0);
        if right_binds_tighter(left, right) {
            let m = p.open_before(lhs);
            p.advance();
            expr_rec(p, right);
            lhs = p.close(m, TreeKind::ExprBinary);
        } else {
            break;
        }
    }
}
fn right_binds_tighter(left: TokenKind, right: TokenKind) -> bool {
    fn tightness(kind: TokenKind) -> Option<usize> {
        [
            // Precedence table:
            [TokenKind::Plus, TokenKind::Minus].as_slice(),
            &[TokenKind::Star, TokenKind::Slash],
        ]
        .iter()
        .position(|level| level.contains(&kind))
    }
    let Some(right_tightness) = tightness(right) else {
    return false
  };
    let Some(left_tightness) = tightness(left) else {
    assert!(left == TokenKind::Eof);
    return true;
  };
    right_tightness > left_tightness
}

// ArgList = '(' Arg* ')'
fn arg_list(p: &mut Parser) {
    tracing::debug!(
        "{}",
        &format!("{} {:?} {} {}", "PARSER".yellow(), p.nth(0), "->".yellow(), "arg_list".green())
    );
    assert!(p.at(TokenKind::LParen));
    let m = p.open();
    p.expect(TokenKind::LParen);
    while !p.at(TokenKind::RParen) && !p.eof() {
        arg(p);
    }
    p.expect(TokenKind::RParen);
    p.close(m, TreeKind::ArgList);
}
// Arg = Expr ','?
fn arg(p: &mut Parser) {
    let m = p.open();
    expr(p);
    if !p.at(TokenKind::RParen) {
        p.expect(TokenKind::Comma);
    }
    p.close(m, TreeKind::Arg);
}

// ExprDelimited =
//   ExprLiteral // true, false, 42
// | ExprName    // foo
// | ExprParen  // (Expr)
// | ExprUnary  // -Expr
fn expr_delimited(p: &mut Parser) -> Option<MarkClosed> {
    let result = match p.nth(0) {
        TokenKind::TrueKeyword | TokenKind::FalseKeyword | TokenKind::Int => {
            tracing::debug!(
                "{}",
                &format!(
                    "{} {:?} {} {}",
                    "PARSER".yellow(),
                    p.nth(0),
                    "->".yellow(),
                    "expr_delimited".green()
                )[..]
            );
            let m = p.open();
            p.advance();
            p.close(m, TreeKind::ExprLiteral)
        }
        TokenKind::Name => {
            tracing::debug!(
                "{}",
                &format!(
                    "{} {:?} {} {}",
                    "PARSER".yellow(),
                    p.nth(0),
                    "->".yellow(),
                    "expr_delimited".green()
                )[..]
            );
            let m = p.open();
            p.advance();
            p.close(m, TreeKind::ExprName)
        }
        TokenKind::LParen => {
            tracing::debug!(
                "{}",
                &format!(
                    "{} {:?} {} {}",
                    "PARSER".yellow(),
                    p.nth(0),
                    "->".yellow(),
                    "expr_delimited".green()
                )[..]
            );
            let m = p.open();
            p.expect(TokenKind::LParen);
            expr(p);
            p.expect(TokenKind::RParen);
            p.close(m, TreeKind::ExprParen)
        }
        _ => return None,
    };
    Some(result)
}

// Param = 'name' ':' TypeExpr ','?
fn param(p: &mut Parser) {
    tracing::debug!(
        "{}",
        &format!("{} {:?} {} {}", "PARSER".yellow(), p.nth(0), "->".yellow(), "param".green())[..]
    );
    assert!(p.at(TokenKind::Name));
    let m = p.open();
    p.expect(TokenKind::Name);
    p.expect(TokenKind::Colon);
    type_expr(p);
    if !p.at(TokenKind::RParen) {
        p.expect(TokenKind::Comma);
    }
    p.close(m, TreeKind::Param);
}

// TypeExpr = 'name'
fn type_expr(p: &mut Parser) {
    let m = p.open();
    p.expect(TokenKind::Name);
    p.close(m, TreeKind::TypeExpr);
}

pub fn parse(text: &str) -> Tree {
    let tokens = lex(text);
    // println!("{:#?}", tokens);
    let mut p = Parser::new(tokens);
    file(&mut p);
    p.build_tree()
}

fn main() {
    let subscriber = Subscriber::builder()
        // .with_env_filter(EnvFilter::from_default_env())
        .with_ansi(true)
        .with_max_level(tracing::Level::TRACE)
        .with_line_number(false)
        .with_thread_names(false)
        .without_time() // turn off timestamps
        .finish();

    // Set the subscriber as the default.
    tracing::subscriber::set_global_default(subscriber).expect("failed to set subscriber");

    let text = "fn f1(x: i32,
fn f2(x: i32,, z: i32) {}
fn f3() {
  let x = ;
}";

    // let x = 1

    let cst = parse(text);
    eprintln!("{cst:?}");
    println!("\ntext\n{text:#?}")
}

// fn fib_rec(f1: u32,

// error: this file contains an unclosed delimiter
//    --> crates/resilient_ll/src/main.rs:628:20
//     |
// 628 | fn fib_rec(f1: u32,
//     |           -        ^
//     |           |
//     |           unclosed delimiter

// error: expected one of `->`, `where`, or `{`, found `<eof>`
//    --> crates/resilient_ll/src/main.rs:628:20
//     |
// 628 | fn fib_rec(f1: u32,
//     |                    ^ expected one of `->`, `where`, or `{`

// error: aborting due to 2 previous errors

// fn fib_rec(f1: u32,
