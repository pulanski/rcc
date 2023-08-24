use std::cell::Cell;

use crate::{
    ast::{
        Child,
        Tree,
        TreeKind,
    },
    lexer::{
        Token,
        TokenKind,
        TokenStream,
    },
    token_set::TokenSet,
};
use owo_colors::OwoColorize;
use smartstring::alias::String;

fn large_parser_prefix() -> String {
    format!(
        "\n\n{}\n{}\n{}{}{}\n{}\n{}",
        "  +--------+".black(),
        "  |        |".black(),
        "  |".black(),
        " PARSER ".yellow(),
        "|".black(),
        "  |        |".black(),
        "  +--------+".black(),
    )
    .into()
}

fn small_parser_prefix() -> String {
    format!("\t{}{}{}", "[".black(), " PARSER ".yellow(), "]".black(),).into()
}

fn format_call_stack(call_stack: &[String]) -> String {
    let mut result = String::new();
    // let mut indentation = 0;

    result.push_str("\n");
    for (indentation, call) in call_stack.iter().enumerate() {
        for _ in 1..indentation {
            // for _ in 1..indentation {
            //   result.push_str("  ");
            // }
            result.push_str("  ");
        }

        if indentation == 0 {
            result.push_str(&format!("{call}\n"));
            continue;
        }

        result.push_str(&format!("+-> {call}\n"));
        format!("{call}");
    }

    result
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

pub struct Parser {
    tokens:     TokenStream,
    pos:        usize,
    fuel:       Cell<u32>,
    events:     Vec<Event>,
    call_stack: Vec<ParserCall>,
}

pub struct ParserCall {
    pub(crate) name: String,
    // pub(crate) body: Tree,
}

impl Parser {
    pub fn new(tokens: TokenStream) -> Parser {
        Parser { tokens, pos: 0, fuel: Cell::new(256), events: Vec::new(), call_stack: Vec::new() }
    }

    pub fn enter(&mut self, tree_kind: TreeKind) {
        let node = tree_kind.green();

        // Adds the node onto the parser call stack
        self.call_stack.push(ParserCall {
            name: node.to_string().into(),
            // body: Tree {
            //   kind: TreeKind::ExprName,
            //   children: Vec::new(),
            // },
        });

        // Prints the node to the console
        // TODO: Make this a debug log
        // e.g. [PARSER] (TYPEDEF_KW, 'typdef', 0..7) - Current Call Stack (<node> ->
        // <node> -> <node>)

        let call_stack_nodes = self
            .call_stack
            .iter()
            .map(|call| {
                let call_name = call.name.clone().to_string();

                format!("{}{}{}", "<".black(), call_name.cyan(), ">".black()).into()
            })
            .collect::<Vec<String>>();
        // .join(&" -> ".yellow().to_string());

        let current_token = &format!(
            " {}{}{} {}{}{}{}{}{}",
            "(".black().italic(),
            self.current().blue(),
            ",".black(),
            "`".red(),
            self.current_token().lexeme(),
            "`".red(),
            ",".black(),
            self.current_token().span().italic(),
            ")".black().italic(),
        );

        // debug the call stack (e.g. <node> -> <node> -> <node>)
        // let call_stack_lines: String = call_stack
        //   .lines()
        //   .collect::<Vec<&str>>()
        //   .join("\n\t\t")
        //   .into();

        let fmt_call_stack = false; // TODO: Make this a debug flag via cli and configuration

        let call_stack = match fmt_call_stack {
            true => format_call_stack(&call_stack_nodes),
            false => node.to_string().into(),
        };

        // tracing::debug!(
        //   "{call_stack} {current_token}",
        //   call_stack = call_stack,
        //   current_token = current_token
        // );

        // tracing::debug!(
        //   "{}\n\n\t{}{}{}{}{}{}{}",
        //   if display_call_stack {
        //     large_parser_prefix()
        //   } else {
        //     "".to_string().into()
        //   },
        //   "CURRENT TOKEN".yellow(),
        //   ":".black(),
        //   current_token,
        //   if display_call_stack {
        //     "\n\n\t\tCALL STACK".yellow().to_string()
        //   } else {
        //     "\n\n  CALL STACK".yellow().to_string()
        //   },
        //   ":".black().to_string(),
        //   if display_call_stack {
        //     "".to_string()
        //   } else {
        //     call_stack_nodes.join(&" -> ".yellow().to_string())
        //   },
        //   if display_call_stack {
        //     let call_stack_str = call_stack.to_string();
        //     call_stack_str.replace('\n', "\n\t\t")
        //   } else {
        //     "".to_string()
        //   },
        //   // call_stack_lines,
        // );

        tracing::debug!(
            "{}",
            &format!(
                "{} {:?} {} {}",
                "PARSER".yellow(),
                self.current_token().kind(),
                "->".yellow(),
                node.green()
            )
        );
    }

    pub fn error(&mut self, message: &str) {
        let m = self.open();
        tracing::error!("{}", message);
        self.close(m, TreeKind::ErrorTree);
    }

    pub fn trace_exit(&mut self) {
        // Removes the node from the parser call stack
        let ast_node = self.call_stack.pop();

        // // Prints the node to the console
        // tracing::debug!(
        //   "{} {} {}",
        //   small_parser_prefix(),
        //   "EXIT"
        //     .yellow()
        //     .italic()
        //     .to_string()
        //     .on_black()
        //     .bold()
        //     .italic(),
        //   ast_node.unwrap().name,
        // );
    }

    fn at_declaration(&self) -> bool {
        self.at_any(&[
            TokenKind::TYPEDEF_KW,
            TokenKind::EXTERN_KW,
            TokenKind::STATIC_KW,
            TokenKind::AUTO_KW,
            TokenKind::REGISTER_KW,
            TokenKind::CONST_KW,
            TokenKind::VOLATILE_KW,
            TokenKind::VOID_KW,
            TokenKind::CHAR_KW,
            TokenKind::SHORT_KW,
            TokenKind::INT_KW,
            TokenKind::LONG_KW,
            TokenKind::FLOAT_KW,
            TokenKind::DOUBLE_KW,
            TokenKind::SIGNED_KW,
            TokenKind::UNSIGNED_KW,
            TokenKind::STRUCT_KW,
            TokenKind::UNION_KW,
            TokenKind::ENUM_KW,
        ])
    }

    fn at_constant(&self) -> bool {
        self.at_any(&[
            TokenKind::INTEGER_CONSTANT,
            TokenKind::FLOATING_CONSTANT,
            TokenKind::IDENTIFIER,
        ])
    }

    fn at_string(&self) -> bool {
        self.at_any(&[TokenKind::STRING, TokenKind::FUNC_NAME_KW])
    }

    fn at_type_qualifier(&self) -> bool {
        self.at_any(&[
            TokenKind::CONST_KW,
            TokenKind::RESTRICT_KW,
            TokenKind::VOLATILE_KW,
            TokenKind::ATOMIC_KW,
        ])
    }

    pub fn log(&self, tree: &Tree) {
        let mut indent = 0;
        let mut stack = Vec::new();
        stack.push(tree);
        while let Some(tree) = stack.pop() {
            for _ in 0..indent {
                print!("  ");
            }
            println!(
                "{}",
                format!(
                    "{}{}",
                    tree.kind,
                    if tree.children.is_empty() { "".to_string() } else { " {".to_string() }
                )
                .blue()
            );
            indent += 1;
            for child in tree.children.iter().rev() {
                match child {
                    Child::Tree(tree) => stack.push(tree),
                    Child::Token(token) => {
                        for _ in 0..indent {
                            print!("  ");
                        }
                        println!("{}", format!("{}", token.kind).green());
                    }
                }
            }
            indent -= 1;
            if !tree.children.is_empty() {
                for _ in 0..indent {
                    print!("  ");
                }
                println!("}}");
            }
        }
    }

    pub fn build_tree(self) -> Tree {
        let mut tokens = self.tokens;
        let mut events = self.events;
        let mut stack = Vec::new();

        // println!("events: {:?}", events);
        // println!("tokens: {:?}", tokens);

        // Special case: pop the last `Close` event to ensure
        // that the stack is non-empty inside the loop.
        assert!(matches!(events.pop(), Some(Event::Close)));

        // println!("events: {:?}", events);

        for event in events {
            // println!("event: {:?}", event);
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
                    // println!("token: {:?}", token);
                    stack.last_mut().unwrap().children.push(Child::Token(token));
                }
            }
        }

        let tree = stack.pop().unwrap();

        // Bump over the EOF token if it exists (it should) in the token stream.
        let eof_token = tokens.next();
        assert!(eof_token.is_some(), "EOF not found");
        // println!("{tokens:#?}");
        // assert!(
        //   eof_token.is_some()
        //     && eof_token.unwrap().kind() == &TokenKind::EOF,
        //   "EOF not found"
        // );

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
        self.at(TokenKind::EOF) || self.pos == self.tokens.len()
    }

    fn nth(&self, lookahead: usize) -> TokenKind {
        if self.fuel.get() == 0 {
            panic!("parser is stuck")
        }
        self.fuel.set(self.fuel.get() - 1);
        self.tokens.get(self.pos + lookahead).map_or(TokenKind::EOF, |it| it.kind)
    }

    fn at(&self, kind: TokenKind) -> bool {
        self.nth(0) == kind
    }

    /// Checks if the current token is in contained within the
    /// given [`TokenSet`], `kinds`.
    pub(crate) fn at_ts(&self, kinds: TokenSet) -> bool {
        tracing::trace!("Checking if current token is in {:?}", kinds);
        kinds.contains(self.current())
    }

    /// Returns the [`TokenKind`] of the current token or **EOF** if the parser
    /// has reached the **end** of the input.
    pub(crate) fn current(&self) -> TokenKind {
        self.nth(0)
    }

    pub fn current_token(&self) -> Token {
        self.tokens.get(self.pos).unwrap()
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
            expected = kind.to_string().yellow(),
            comma = ",".black(),
            found = found.red(),
            period = ".".black(),
        );
    }

    fn at_static_assert_declaration(&self) -> bool {
        self.at(TokenKind::STATIC_ASSERT_KW)
    }

    fn at_declaration_specifier(&self) -> bool {
        self.at_storage_class_specifier() ||
            self.at_type_specifier() ||
            self.at_type_qualifier() ||
            self.at_function_specifier() ||
            self.at_alignment_specifier()
    }

    fn at_alignment_specifier(&self) -> bool {
        self.at(TokenKind::ALIGNAS_KW)
    }

    fn at_type_specifier(&self) -> bool {
        self.at_any(&[
            TokenKind::VOID_KW,
            TokenKind::CHAR_KW,
            TokenKind::SHORT_KW,
            TokenKind::INT_KW,
            TokenKind::LONG_KW,
            TokenKind::FLOAT_KW,
            TokenKind::DOUBLE_KW,
            TokenKind::SIGNED_KW,
            TokenKind::UNSIGNED_KW,
            TokenKind::BOOL_KW,
            TokenKind::COMPLEX_KW,
            TokenKind::IMAGINARY_KW,
            TokenKind::ENUM_KW,
            TokenKind::ATOMIC_KW,
            TokenKind::STRUCT_KW,
            TokenKind::UNION_KW,
            // TODO: figure out what these are and how to handle them
            // TokenKind::TYPEDEF_NAME,
        ])
    }

    fn at_generic_selection(&self) -> bool {
        self.at(TokenKind::GENERIC_KW)
    }

    fn at_storage_class_specifier(&self) -> bool {
        self.at_any(&[
            TokenKind::TYPEDEF_KW,
            TokenKind::EXTERN_KW,
            TokenKind::STATIC_KW,
            TokenKind::THREAD_LOCAL_KW,
            TokenKind::AUTO_KW,
            TokenKind::REGISTER_KW,
        ])
    }

    fn at_function_specifier(&self) -> bool {
        self.at_any(&[TokenKind::INLINE_KW, TokenKind::NORETURN_KW])
    }

    fn add_leaf(&mut self, kind: TreeKind) {
        let m = self.open();
        self.advance();
        self.close(m, kind);
    }

    fn expect_any(&mut self, kinds: &[TokenKind]) {
        let found = self.nth(0);
        if self.at_any(kinds) {
            self.advance();
            return;
        }

        // TODO: Error reporting.

        tracing::error!(
            "Expected {expected}{comma} but instead found {found}{period}",
            expected =
                kinds.iter().map(|it| it.to_string()).collect::<Vec<_>>().join(" or ").yellow(),
            comma = ",".black(),
            found = found.red(),
            period = ".".black(),
        );
    }

    fn at_assignment_operator(&self) -> bool {
        self.at_any(&[
            TokenKind::EQ,
            TokenKind::STAREQ,
            TokenKind::SLASHEQ,
            TokenKind::PERCENTEQ,
            TokenKind::PLUSEQ,
            TokenKind::MINUSEQ,
            TokenKind::LSHIFTEQ,
            TokenKind::RSHIFTEQ,
            TokenKind::AMPEQ,
            TokenKind::CARETEQ,
            TokenKind::PIPEEQ,
        ])
    }

    fn at_statement(&self) -> bool {
        self.at_any(STATEMENT_LIST_FIRST)
    }

    fn at_constant_expression(&self) -> bool {
        self.at_primary_expression()
    }

    fn at_primary_expression(&self) -> bool {
        self.at_any(&[
            TokenKind::IDENTIFIER,
            // TokenKind::CONSTANT,
            TokenKind::STRING,
            TokenKind::LPAREN,
        ])
        // TODO: update to 2011 standard
        // || self.at_constant()
    }

    fn at_unary_operator(&self) -> bool {
        self.at_any(&[
            TokenKind::AMP,
            TokenKind::STAR,
            TokenKind::PLUS,
            TokenKind::MINUS,
            TokenKind::TILDE,
            TokenKind::BANG,
        ])
    }

    fn peek(&mut self) -> Option<Token> {
        self.tokens.get(self.pos)
    }
}

// translation_unit
// : external_declaration
// | translation_unit external_declaration
// ;
//
// TranslationUnit = ExternDecl*
pub fn translation_unit(p: &mut Parser) {
    p.enter(TreeKind::TranslationUnit);
    let m = p.open();

    // // flag to indicate whether we are parsing the first declaration
    // // let mut first = true;

    // // parse all external declarations
    // while !p.eof() {
    //   // TODO: Error recovery.
    //   // if first {
    //   //   first = false;
    //   // } else {
    //   //   p.expect(TokenKind::SEMI);
    //   // }

    //   extern_decl(p);
    // }

    let mut seen_extern = false;

    loop {
        // If there are no more tokens, break
        if p.peek().is_none() {
            if !seen_extern {
                p.advance_with_error("expected 'main' function");
            }
            break;
        }

        // Parse an external declaration
        extern_decl(p);

        // If flag isn't set, set it
        if !seen_extern {
            seen_extern = true;
        }
    }

    p.close(m, TreeKind::TranslationUnit);
    p.trace_exit()
}

const FN_DEF_DECLARATION_SPECIFIERS_FIRST: &[TokenKind] = &[
    TokenKind::VOID_KW,
    TokenKind::CHAR_KW,
    TokenKind::SHORT_KW,
    TokenKind::INT_KW,
    TokenKind::LONG_KW,
    TokenKind::FLOAT_KW,
    TokenKind::DOUBLE_KW,
    TokenKind::SIGNED_KW,
    TokenKind::UNSIGNED_KW,
    TokenKind::STRUCT_KW,
    TokenKind::UNION_KW,
    TokenKind::ENUM_KW,
    TokenKind::IDENTIFIER,
];
// FN_DEF_FIRST = FN_DEF_DECLARATION_SPECIFIERS_FIRST | FN_DEF_DECLARATOR_FIRST
// const FN_DEF_FIRST: &[TokenKind] = // TODO:

// external_declaration
// : function_definition
// | declaration
// ;
//
// ExternDecl = FunctionDef | Declaration
fn extern_decl(p: &mut Parser) {
    p.enter(TreeKind::ExternDecl);
    let m = p.open();

    if p.at_any(FN_DEF_DECLARATION_SPECIFIERS_FIRST) {
        function_def(p);
    } else {
        declaration(p);
    }

    p.close(m, TreeKind::ExternDecl);
    p.trace_exit();
}

// function_definition
// 	: declaration_specifiers declarator declaration_list compound_statement
// 	| declaration_specifiers declarator compound_statement
// 	;
//
// FunctionDef = DeclarationSpecifiers Declarator (DeclarationList)?
// CompoundStatement
// | DeclarationSpecifiers Declarator CompoundStatement
pub(crate) fn function_def(p: &mut Parser) {
    p.enter(TreeKind::FunctionDef);
    let m = p.open();

    // Parse declaration specifiers
    declaration_specifiers(p);

    println!("parsing declarator in function_def {:?}", p.current_token());

    // Parse declarator
    declarator(p);

    println!("finished parsing declarator in function_def {:?}", p.current_token());

    // Check if there's a declaration list
    if p.at_any(DECLARATION_LIST_FIRST) {
        // Parse the declaration list
        declaration_list(p);
    }

    // Parse the compound statement
    compound_statement(p);

    p.close(m, TreeKind::FunctionDef);
    p.trace_exit();
}

const STATEMENT_LIST_FIRST: &[TokenKind] = &[
    TokenKind::IDENTIFIER,
    TokenKind::IF_KW,
    TokenKind::WHILE_KW,
    TokenKind::FOR_KW,
    TokenKind::RETURN_KW,
    TokenKind::BREAK_KW,
    TokenKind::CONTINUE_KW,
    TokenKind::LBRACE,
];

const DECLARATION_LIST_FIRST: &[TokenKind] = &[
    TokenKind::IDENTIFIER,
    TokenKind::TYPEDEF_KW,
    TokenKind::EXTERN_KW,
    TokenKind::STATIC_KW,
    TokenKind::AUTO_KW,
    TokenKind::REGISTER_KW,
    TokenKind::CONST_KW,
    TokenKind::VOLATILE_KW,
    TokenKind::VOID_KW,
    TokenKind::CHAR_KW,
    TokenKind::SHORT_KW,
    TokenKind::INT_KW,
    TokenKind::LONG_KW,
    TokenKind::FLOAT_KW,
    TokenKind::DOUBLE_KW,
    TokenKind::SIGNED_KW,
    TokenKind::UNSIGNED_KW,
    TokenKind::STRUCT_KW,
    TokenKind::UNION_KW,
    TokenKind::ENUM_KW,
];

// statement_list
// : statement
// | statement_list statement
// ;
//
// StatementList = (Statement)* Statement
fn statement_list(p: &mut Parser) {
    p.enter(TreeKind::StatementList);
    let m = p.open();

    // Parse statements until we reach the end of the file or a closing brace
    while !p.eof() && !p.at(TokenKind::RBRACE) {
        // Attempt to parse a statement
        if !p.at_statement() {
            // If parsing a statement fails, report an error and try to recover
            p.error("Expected a statement");
            // Attempt to skip tokens until we find a statement or reach the end
            while !p.eof() && !p.at(TokenKind::RBRACE) {
                if p.at_statement() {
                    // If we successfully parse a statement while recovering, continue
                    break;
                } else {
                    // Otherwise, advance to the next token
                    p.advance();
                }
            }
        }

        // Parse a statement
        statement(p);
    }

    p.close(m, TreeKind::StatementList);
    p.trace_exit();
}

// statement
// : labeled_statement
// | compound_statement
// | expression_statement
// | selection_statement
// | iteration_statement
// | jump_statement
// ;
//
// Statement = ExpressionStatement
// | CompoundStatement
// | ExpressionStatement
// | SelectionStatement
// | IterationStatement
// | JumpStatement
pub(crate) fn statement(p: &mut Parser) {
    p.enter(TreeKind::Statement);

    // println!("parsing statement: {:?}", p.current_token());

    let m = p.open();

    if p.at_any(&[TokenKind::CASE_KW, TokenKind::DEFAULT_KW]) ||
        (p.at(TokenKind::IDENTIFIER) && p.nth(1) == TokenKind::COLON)
    {
        labeled_statement(p);
    } else if p.at(TokenKind::LBRACE) {
        compound_statement(p);
    } else if p.at(TokenKind::IF_KW) {
        selection_statement(p);
    } else if p.at_any(&[TokenKind::WHILE_KW, TokenKind::FOR_KW, TokenKind::DO_KW]) {
        iteration_statement(p);
    } else if p.at_any(&[
        TokenKind::RETURN_KW,
        TokenKind::BREAK_KW,
        TokenKind::CONTINUE_KW,
        TokenKind::GOTO_KW,
    ]) {
        jump_statement(p);
    } else if p.at_any(&[TokenKind::IDENTIFIER, TokenKind::SEMICOLON]) {
        expression_statement(p);
    } else {
        p.advance_with_error("Expected statement");
    }

    p.close(m, TreeKind::Statement);
    p.trace_exit();
}

// labeled_statement
// : IDENTIFIER ':' statement
// | CASE_KW constant_expression ':' statement
// | DEFAULT_KW ':' statement
// ;
//
// LabeledStatement = IDENTIFIER ':' Statement
// | CASE_KW ConstantExpression ':' Statement
// | DEFAULT_KW ':' Statement
fn labeled_statement(p: &mut Parser) {
    p.enter(TreeKind::LabeledStatement);
    let m = p.open();

    if p.at_any(&[TokenKind::IDENTIFIER, TokenKind::DEFAULT_KW]) {
        p.advance();
        p.expect(TokenKind::COLON);
        statement(p);
    } else if p.at(TokenKind::CASE_KW) {
        p.advance();
        constant_expression(p);
        p.expect(TokenKind::COLON);
        statement(p);
    } else {
        p.advance_with_error("Expected IDENTIFIER, CASE_KW, or DEFAULT_KW in labeled_statement");
    }

    p.close(m, TreeKind::LabeledStatement);
    p.trace_exit();
}

// compound_statement
// : '{' '}'
// | '{' statement_list '}'
// | '{' declaration_list '}'
// | '{' declaration_list statement_list '}'
// ;
//
// CompoundStatement = '{' (DeclarationList)? (StatementList)? '}'
pub(crate) fn compound_statement(p: &mut Parser) {
    assert!(
        p.at(TokenKind::LBRACE),
        r#"Expected '{{' in compound_statement. Found: "{}". This is a bug. Please report it at https://github.com/pulanski/rcc/issues/new."#,
        p.current_token().lexeme()
    ); // Invariant.

    p.enter(TreeKind::CompoundStatement);
    let m = p.open();

    p.expect(TokenKind::LBRACE);

    if p.at_any(STATEMENT_LIST_FIRST) {
        statement_list(p);
    } else if p.at_any(DECLARATION_LIST_FIRST) {
        declaration_list(p);

        // Check if there's a statement_list after declaration_list
        if p.at_any(STATEMENT_LIST_FIRST) {
            statement_list(p);
        }
    }

    p.expect(TokenKind::RBRACE);

    p.close(m, TreeKind::CompoundStatement);
    p.trace_exit();
}

// expression_statement
// : ';'
// | expression ';'
// ;
//
// ExpressionStatement = (Expression)? ';'
pub(crate) fn expression_statement(p: &mut Parser) {
    p.enter(TreeKind::ExpressionStatement);
    let m = p.open();

    if p.at(TokenKind::SEMICOLON) {
        p.advance();
    } else {
        expression(p);
        p.expect(TokenKind::SEMICOLON);
    }

    p.close(m, TreeKind::ExpressionStatement);
    p.trace_exit();
}

// selection_statement
// : IF_KW '(' expression ')' statement
// | IF_KW '(' expression ')' statement ELSE_KW statement
// | SWITCH_KW '(' expression ')' statement
// ;
//
// SelectionStatement = IF_KW '(' Expression ')' Statement
// | IF_KW '(' Expression ')' Statement ELSE_KW Statement
// | SWITCH_KW '(' Expression ')' Statement
pub(crate) fn selection_statement(p: &mut Parser) {
    p.enter(TreeKind::SelectionStatement);
    let m = p.open();

    if p.at(TokenKind::IF_KW) {
        p.advance();
        p.expect(TokenKind::LPAREN);
        expression(p);
        p.expect(TokenKind::RPAREN);
        statement(p);

        if p.at(TokenKind::ELSE_KW) {
            p.advance();
            statement(p);
        }
    } else if p.at(TokenKind::SWITCH_KW) {
        p.advance();
        p.expect(TokenKind::LPAREN);
        expression(p);
        p.expect(TokenKind::RPAREN);
        statement(p);
    } else {
        p.advance_with_error("Expected IF_KW or SWITCH_KW in selection_statement");
    }

    p.close(m, TreeKind::SelectionStatement);
    p.trace_exit();
}

/// Parses an **iteration statement** as per the [**C grammar**][1].
///
/// # Syntax
/// ```text
/// iteration_statement
///     : WHILE '(' expression ')' statement
///     | DO statement WHILE '(' expression ')' ';'
///     | FOR '(' expression_statement expression_statement ')' statement
///     | FOR '(' expression_statement expression_statement expression ')' statement
///     | FOR '(' declaration expression_statement ')' statement
///     | FOR '(' declaration expression_statement expression ')' statement
///     ;
/// ```
///
/// ```ungrammar
/// IterationStatement = "while" "(" Expression ")" Statement
/// | "do" Statement "while" "(" Expression ")" ";"
/// | "for" "(" ExpressionStatement ExpressionStatement Expression? ")" Statement
/// | "for" "(" Declaration ExpressionStatement Expression? ")" Statement
/// ```
///
/// # Arguments
/// * `p` - A mutable reference to the parser.
///
/// # Notes
/// This function assumes that the parser is positioned at the start of an
/// iteration statement.
///
/// [1]: https://port70.net/~nsz/c/c11/n1570.html#6.8.5
pub(crate) fn iteration_statement(p: &mut Parser) {
    p.enter(TreeKind::IterationStatement);
    let m = p.open();

    if p.at(TokenKind::WHILE_KW) {
        // Parse a 'while' loop
        p.advance();
        p.expect(TokenKind::LPAREN);
        expression(p);
        p.expect(TokenKind::RPAREN);
        statement(p);
    } else if p.at(TokenKind::DO_KW) {
        // Parse a 'do...while' loop
        p.advance();
        statement(p);
        p.expect(TokenKind::WHILE_KW);
        p.expect(TokenKind::LPAREN);
        expression(p);
        p.expect(TokenKind::RPAREN);
        p.expect(TokenKind::SEMICOLON);
    } else if p.at(TokenKind::FOR_KW) {
        // Parse a 'for' loop
        p.advance();
        p.expect(TokenKind::LPAREN);

        // Check for declaration or expression_statement
        if p.at_declaration() {
            // Parse a declaration
            declaration(p);
        } else if !p.at(TokenKind::SEMICOLON) {
            // Parse the first expression_statement
            expression_statement(p);
        }

        // Parse the second expression_statement
        if !p.at(TokenKind::SEMICOLON) {
            expression_statement(p);
        }

        // Parse the optional third expression
        if !p.at(TokenKind::RPAREN) {
            expression(p);
        }

        p.expect(TokenKind::RPAREN);
        statement(p);
    } else {
        p.advance_with_error("Expected WHILE_KW, DO_KW, or FOR_KW in iteration_statement");
    }

    p.close(m, TreeKind::IterationStatement);
    p.trace_exit();
}

// atomic_type_specifier
// : ATOMIC_KW '(' type_name ')'
// ;
//
// AtomicTypeSpecifier = ATOMIC_KW '(' TypeName ')'
fn atomic_type_specifier(p: &mut Parser) {
    p.enter(TreeKind::AtomicTypeSpecifier);
    let m = p.open();

    p.expect(TokenKind::ATOMIC_KW);
    p.expect(TokenKind::LPAREN);
    type_name(p);
    p.expect(TokenKind::RPAREN);

    p.close(m, TreeKind::AtomicTypeSpecifier);
    p.trace_exit();
}

// jump_statement
// : RETURN_KW expression? ';'
// | BREAK_KW ';'
// | CONTINUE_KW ';'
// | GOTO_KW IDENTIFIER ';'
// ;
//
// JumpStatement = RETURN_KW (Expression)? ';'
// | BREAK_KW ';'
// | CONTINUE_KW ';'
// | GOTO_KW IDENTIFIER ';'
pub(crate) fn jump_statement(p: &mut Parser) {
    p.enter(TreeKind::JumpStatement);
    let m = p.open();

    if p.at(TokenKind::RETURN_KW) {
        p.advance();
        if !p.at(TokenKind::SEMICOLON) {
            expression(p);
        }
        p.expect(TokenKind::SEMICOLON);
    } else if p.at_any(&[TokenKind::BREAK_KW, TokenKind::CONTINUE_KW]) {
        p.advance();
        p.expect(TokenKind::SEMICOLON);
    } else if p.at(TokenKind::GOTO_KW) {
        p.advance();
        p.expect(TokenKind::IDENTIFIER);
        p.expect(TokenKind::SEMICOLON);
    } else {
        p.advance_with_error(
            "Expected RETURN_KW, BREAK_KW, CONTINUE_KW, or GOTO_KW in jump_statement",
        );
    }

    p.close(m, TreeKind::JumpStatement);
    p.trace_exit();
}

// declaration
// 	: declaration_specifiers ';'
// 	| declaration_specifiers init_declarator_list ';'
// 	| static_assert_declaration
// 	;
//
// Declaration = DeclarationSpecifiers ';'
// | DeclarationSpecifiers InitDeclaratorList ';'
// | StaticAssertDeclaration
pub(crate) fn declaration(p: &mut Parser) {
    p.enter(TreeKind::Declaration);
    let m = p.open();

    // println!("parsing declaration: {:?}", p.current_token());
    if p.at(TokenKind::STATIC_ASSERT_KW) {
        static_assert_declaration(p);
    } else {
        declaration_specifiers(p);
        if p.at(TokenKind::SEMICOLON) {
            p.advance();
        } else {
            init_declarator_list(p);
            p.expect(TokenKind::SEMICOLON);
        }
    }

    p.close(m, TreeKind::Declaration);
    p.trace_exit();
}

// static_assert_declaration
// 	: STATIC_ASSERT '(' constant_expression ',' STRING_LITERAL ')' ';'
// 	;
//
// StaticAssertDeclaration = STATIC_ASSERT '(' ConstantExpression ','
// STRING_LITERAL ')' ';'
fn static_assert_declaration(p: &mut Parser) {
    p.enter(TreeKind::StaticAssertDeclaration);
    let m = p.open();

    p.expect(TokenKind::STATIC_ASSERT_KW);
    p.expect(TokenKind::LPAREN);
    constant_expression(p);
    p.expect(TokenKind::COMMA);
    p.expect(TokenKind::STRING);
    p.expect(TokenKind::RPAREN);
    p.expect(TokenKind::SEMICOLON);

    p.close(m, TreeKind::StaticAssertDeclaration);
    p.trace_exit();
}

/// ```yacc
/// declarator
/// : pointer? direct_declarator
/// ;
///
/// Declarator = (Pointer)? DirectDeclarator
/// ```
fn declarator(p: &mut Parser) {
    p.enter(TreeKind::Declarator);
    let m = p.open();

    if p.at(TokenKind::STAR) {
        pointer(p);
    }
    direct_declarator(p);

    p.close(m, TreeKind::Declarator);
    p.trace_exit();
}

const PARAMETER_TYPE_LIST_FIRST: &[TokenKind] = &[
    TokenKind::VOID_KW,
    TokenKind::CHAR_KW,
    TokenKind::SHORT_KW,
    TokenKind::INT_KW,
    TokenKind::LONG_KW,
    // TokenKind::FLOAT,
    TokenKind::DOUBLE_KW,
    TokenKind::SIGNED_KW,
    TokenKind::UNSIGNED_KW,
    TokenKind::STRUCT_KW,
    TokenKind::UNION_KW,
    TokenKind::ENUM_KW,
    TokenKind::IDENTIFIER,
];

// direct_declarator
// 	: IDENTIFIER
// 	| '(' declarator ')'
// 	| direct_declarator '[' ']'
// 	| direct_declarator '[' '*' ']'
// 	| direct_declarator '[' STATIC type_qualifier_list assignment_expression ']'
// 	| direct_declarator '[' STATIC assignment_expression ']'
// 	| direct_declarator '[' type_qualifier_list '*' ']'
// 	| direct_declarator '[' type_qualifier_list STATIC assignment_expression ']'
// 	| direct_declarator '[' type_qualifier_list assignment_expression ']'
// 	| direct_declarator '[' type_qualifier_list ']'
// 	| direct_declarator '[' assignment_expression ']'
// 	| direct_declarator '(' parameter_type_list ')'
// 	| direct_declarator '(' ')'
// 	| direct_declarator '(' identifier_list ')'
// 	;
//
// DirectDeclarator = IDENTIFIER
// | '(' Declarator ')'
// | DirectDeclarator '[' '*'? ']'
// | DirectDeclarator '[' STATIC TypeQualifierList AssignmentExpression ']'
// | DirectDeclarator '[' STATIC AssignmentExpression ']'
// | DirectDeclarator '[' TypeQualifierList '*'? ']'
// | DirectDeclarator '[' TypeQualifierList STATIC? AssignmentExpression ']'
// | DirectDeclarator '[' AssignmentExpression ']'
// | DirectDeclarator '(' ParameterTypeList ')'
// | DirectDeclarator '(' ')'
// | DirectDeclarator '(' IdentifierList ')'
pub(crate) fn direct_declarator(p: &mut Parser) {
    // TODO: need to come back to this and update potentially
    p.enter(TreeKind::DirectDeclarator);
    let m = p.open();

    println!("parsing direct_declarator: {:?}", p.current_token());

    // Try to match the various production rules for direct_declarator
    if p.at(TokenKind::IDENTIFIER) {
        p.advance(); // Consume IDENTIFIER

        // Check if it's a function call
        if p.at(TokenKind::LPAREN) {
            p.advance(); // Consume '('

            // Check if it's a parameter_type_list or identifier_list
            if p.at_any(PARAMETER_TYPE_LIST_FIRST) {
                parameter_type_list(p);
            } else if p.at(TokenKind::IDENTIFIER) {
                identifier_list(p);
            }

            p.expect(TokenKind::RPAREN); // Consume ')'
        }
    } else if p.at(TokenKind::LPAREN) {
        p.advance(); // Consume '('
        declarator(p);
        p.expect(TokenKind::RPAREN);
    } else {
        // Recursively parse the direct_declarator until we reach the base case
        direct_declarator(p);

        // p.expect(TokenKind::LBRACKET);

        if p.at(TokenKind::LPAREN) {
            p.advance(); // Consume '('

            if p.at_any(PARAMETER_TYPE_LIST_FIRST) {
                parameter_type_list(p);
            } else if p.at(TokenKind::IDENTIFIER) {
                identifier_list(p);
            }

            p.expect(TokenKind::RPAREN); // Consume ')'
        } else if p.at(TokenKind::LBRACKET) {
            if p.at(TokenKind::STAR) {
                p.advance(); // Consume '*'
                p.expect(TokenKind::RBRACKET);
            } else if p.at(TokenKind::STATIC_KW) {
                p.advance(); // Consume STATIC

                if p.at_type_qualifier() {
                    type_qualifier_list(p);
                } else if p.at_assignment_operator() {
                    assignment_expression(p);
                } else {
                    // TODO: error handling
                    p.advance_with_error(&format!(
                        "Expected type_qualifier_list or assignment_expression in \
                         direct_declarator, but instead found `{:?}`",
                        p.current_token().lexeme()
                    ));
                }
            }

            // if p.at(TokenKind::STATIC_KW) {
            //     p.advance(); // Consume STATIC
            // }
        } else {
            // TODO: error handling
            p.advance_with_error(&format!(
                "Expected LPAREN or LBRACKET in direct_declarator, but instead found {:?}",
                p.current_token()
            ));
        }

        p.expect(TokenKind::RBRACKET);
    }

    p.close(m, TreeKind::DirectDeclarator);
    p.trace_exit();
}

// p.trace_enter("direct_declarator");
// let m = p.open();

// if p.at(TokenKind::IDENTIFIER) {
//   p.advance(); // Consume the IDENTIFIER token
// } else if p.at(TokenKind::LPAREN) {
//   p.advance(); // Consume the LPAREN token

//   // Check if it's a parameter_type_list or identifier_list
//   if p.at_any(PARAMETER_TYPE_LIST_FIRST) {
//     parameter_type_list(p);
//   } else {
//     identifier_list(p);
//   }

//   p.expect(TokenKind::RPAREN); // Consume the RPAREN token
// } else if p.at(TokenKind::LBRACKET) {
//   p.advance(); // Consume the LBRACKET token
//   constant_expression(p); // Parse the constant_expression if present
//   p.expect(TokenKind::RBRACKET); // Consume the RBRACKET token
// } else {
//   // Handle other cases or report an error
//   // For example, you might want to report an error here
//   p.advance_with_error("Expected IDENTIFIER, LPAREN, or LBRACKET in
// direct_declarator"); }

// p.close(m, TreeKind::DirectDeclarator);
// p.trace_exit();

// identifier_list
// : IDENTIFIER
// | identifier_list ',' IDENTIFIER
// ;
//
// IdentifierList = IDENTIFIER
// | IdentifierList ',' IDENTIFIER
fn identifier_list(p: &mut Parser) {
    p.enter(TreeKind::IdentifierList);
    let m = p.open();

    while !p.eof() && !p.at(TokenKind::RPAREN) {
        // TODO: Error recovery.
    }

    p.close(m, TreeKind::IdentifierList);
    p.trace_exit();
}

// constant_expression
// : conditional_expression
// ;
//
// ConstantExpression = ConditionalExpression
fn constant_expression(p: &mut Parser) {
    p.enter(TreeKind::ConstantExpression);
    let m = p.open();

    conditional_expression(p);

    p.close(m, TreeKind::ConstantExpression);
    p.trace_exit();
}

// conditional_expression
// : logical_or_expression
// | logical_or_expression '?' expression ':' conditional_expression
// ;
//
// ConditionalExpression = LogicalOrExpression
// | LogicalOrExpression '?' Expression ':' ConditionalExpression
fn conditional_expression(p: &mut Parser) {
    p.enter(TreeKind::ConditionalExpression);
    let m = p.open();

    logical_or_expression(p);
    if p.at(TokenKind::QUESTION) {
        p.advance();
        expression(p);
        p.expect(TokenKind::COLON);
        conditional_expression(p);
    }

    p.close(m, TreeKind::ConditionalExpression);
    p.trace_exit();
}

// expression
// : assignment_expression
// | expression ',' assignment_expression
// ;
//
// Expression = AssignmentExpression
// | Expression ',' AssignmentExpression
fn expression(p: &mut Parser) {
    p.enter(TreeKind::Expression);
    let m = p.open();

    assignment_expression(p);
    while p.at(TokenKind::COMMA) {
        p.advance();
        assignment_expression(p);
    }

    p.close(m, TreeKind::Expression);
    p.trace_exit();
}

// assignment_expression
// : conditional_expression
// | unary_expression assignment_operator assignment_expression
// ;
//
// AssignmentExpression = ConditionalExpression
// | UnaryExpression AssignmentOperator AssignmentExpression
fn assignment_expression(p: &mut Parser) {
    p.enter(TreeKind::AssignmentExpression);
    let m = p.open();

    conditional_expression(p);
    if p.at_assignment_operator() {
        p.advance();
        assignment_expression(p);
    }

    p.close(m, TreeKind::AssignmentExpression);
    p.trace_exit();
}

// assignment_operator
// : '='
// | MUL_ASSIGN
// | DIV_ASSIGN
// | MOD_ASSIGN
// | ADD_ASSIGN
// | SUB_ASSIGN
// | LEFT_ASSIGN
// | RIGHT_ASSIGN
// | AND_ASSIGN
// | XOR_ASSIGN
// | OR_ASSIGN
// ;
//
// AssignmentOperator = '='
// | MUL_ASSIGN
// | DIV_ASSIGN
// | MOD_ASSIGN
// | ADD_ASSIGN
// | SUB_ASSIGN
// | LEFT_ASSIGN
// | RIGHT_ASSIGN
// | AND_ASSIGN
// | XOR_ASSIGN
// | OR_ASSIGN
fn assignment_operator(p: &mut Parser) {
    // p.expect_assignment_operator();
}

// logical_or_expression
// : logical_and_expression
// | logical_or_expression '||' logical_and_expression
// ;
//
// LogicalOrExpression = LogicalAndExpression
// | LogicalOrExpression '||' LogicalAndExpression
fn logical_or_expression(p: &mut Parser) {
    p.enter(TreeKind::LogicalOrExpression);
    let m = p.open();

    // TODO: refactor to
    // let parsing_routine = p.enter(TreeKind::LogicalOrExpression);
    //
    // // parsing routine here
    // ...
    //
    // p.exit(parsing_routine);
    //

    logical_and_expression(p);
    while p.at(TokenKind::DOUBLEPIPE) {
        p.advance();
        logical_and_expression(p);
    }

    p.close(m, TreeKind::LogicalOrExpression);
}

// logical_and_expression
// : inclusive_or_expression
// | logical_and_expression '&&' inclusive_or_expression
// ;
//
// LogicalAndExpression = InclusiveOrExpression
// | LogicalAndExpression '&&' InclusiveOrExpression
fn logical_and_expression(p: &mut Parser) {
    p.enter(TreeKind::LogicalAndExpression);

    let m = p.open();
    inclusive_or_expression(p);
    while p.at(TokenKind::DOUBLEAMP) {
        p.advance();
        inclusive_or_expression(p);
    }

    p.close(m, TreeKind::LogicalAndExpression);
    p.trace_exit();
}

// inclusive_or_expression
// : exclusive_or_expression
// | inclusive_or_expression '|' exclusive_or_expression
// ;
//
// InclusiveOrExpression = ExclusiveOrExpression
// | InclusiveOrExpression '|' ExclusiveOrExpression
fn inclusive_or_expression(p: &mut Parser) {
    p.enter(TreeKind::InclusiveOrExpression);

    let m = p.open();
    exclusive_or_expression(p);
    while p.at(TokenKind::PIPE) {
        p.advance();
        exclusive_or_expression(p);
    }

    p.close(m, TreeKind::InclusiveOrExpression);
    p.trace_exit();
}

// exclusive_or_expression
// : and_expression
// | exclusive_or_expression '^' and_expression
// ;
//
// ExclusiveOrExpression = AndExpression
// | ExclusiveOrExpression '^' AndExpression
fn exclusive_or_expression(p: &mut Parser) {
    p.enter(TreeKind::ExclusiveOrExpression);

    let m = p.open();
    and_expression(p);
    while p.at(TokenKind::CARET) {
        p.advance();
        and_expression(p);
    }

    p.close(m, TreeKind::ExclusiveOrExpression);
    p.trace_exit();
}

// and_expression
// : equality_expression
// | and_expression '&' equality_expression
// ;
//
// AndExpression = EqualityExpression
// | AndExpression '&' EqualityExpression
fn and_expression(p: &mut Parser) {
    p.enter(TreeKind::AndExpression);

    let m = p.open();
    equality_expression(p);
    while p.at(TokenKind::AMP) {
        p.advance();
        equality_expression(p);
    }

    p.close(m, TreeKind::AndExpression);
    p.trace_exit();
}

// equality_expression
// : relational_expression
// | equality_expression EQ relational_expression
// | equality_expression NE relational_expression
// ;
//
// EqualityExpression = RelationalExpression
// | EqualityExpression EQ RelationalExpression
// | EqualityExpression NE RelationalExpression
fn equality_expression(p: &mut Parser) {
    p.enter(TreeKind::EqualityExpression);
    let m = p.open();

    relational_expression(p);
    while p.at(TokenKind::EQEQ) || p.at(TokenKind::NE) {
        p.advance();
        relational_expression(p);
    }

    p.close(m, TreeKind::EqualityExpression);
    p.trace_exit();
}

// relational_expression
// : shift_expression
// | relational_expression '<' shift_expression
// | relational_expression '>' shift_expression
// | relational_expression LE shift_expression
// | relational_expression GE shift_expression
// ;
//
// RelationalExpression = ShiftExpression
// | RelationalExpression '<' ShiftExpression
// | RelationalExpression '>' ShiftExpression
// | RelationalExpression LE ShiftExpression
// | RelationalExpression GE ShiftExpression
fn relational_expression(p: &mut Parser) {
    let m = p.open();
    shift_expression(p);
    while p.at(TokenKind::LT) || p.at(TokenKind::GT) || p.at(TokenKind::LE) || p.at(TokenKind::GE) {
        p.advance();
        shift_expression(p);
    }

    p.close(m, TreeKind::RelationalExpression);
}

// shift_expression
// : additive_expression
// | shift_expression LEFT_OP additive_expression
// | shift_expression RIGHT_OP additive_expression
// ;
//
// ShiftExpression = AdditiveExpression
// | ShiftExpression LEFT_OP AdditiveExpression
// | ShiftExpression RIGHT_OP AdditiveExpression
fn shift_expression(p: &mut Parser) {
    let m = p.open();
    additive_expression(p);
    while p.at(TokenKind::LSHIFT) || p.at(TokenKind::RSHIFT) {
        p.advance();
        additive_expression(p);
    }

    p.close(m, TreeKind::ShiftExpression);
}

// additive_expression
// : multiplicative_expression
// | additive_expression '+' multiplicative_expression
// | additive_expression '-' multiplicative_expression
// ;
//
// AdditiveExpression = MultiplicativeExpression
// | AdditiveExpression '+' MultiplicativeExpression
// | AdditiveExpression '-' MultiplicativeExpression
fn additive_expression(p: &mut Parser) {
    let m = p.open();
    multiplicative_expression(p);
    while p.at(TokenKind::PLUS) || p.at(TokenKind::MINUS) {
        p.advance();
        multiplicative_expression(p);
    }

    p.close(m, TreeKind::AdditiveExpression);
}

// multiplicative_expression
// : cast_expression
// | multiplicative_expression '*' cast_expression
// | multiplicative_expression '/' cast_expression
// | multiplicative_expression '%' cast_expression
// ;
//
// MultiplicativeExpression = CastExpression
// | MultiplicativeExpression '*' CastExpression
// | MultiplicativeExpression '/' CastExpression
// | MultiplicativeExpression '%' CastExpression
fn multiplicative_expression(p: &mut Parser) {
    let m = p.open();
    cast_expression(p);
    while p.at(TokenKind::STAR) || p.at(TokenKind::SLASH) || p.at(TokenKind::PERCENT) {
        p.advance();
        cast_expression(p);
    }

    p.close(m, TreeKind::MultiplicativeExpression);
}

// cast_expression
// : unary_expression
// | '(' type_name ')' cast_expression
// ;
//
// CastExpression = UnaryExpression
// | '(' TypeName ')' CastExpression
fn cast_expression(p: &mut Parser) {
    p.enter(TreeKind::CastExpression);
    let m = p.open();

    if p.at(TokenKind::LPAREN) {
        p.advance();
        type_name(p);
        p.expect(TokenKind::RPAREN);
        cast_expression(p);
    } else {
        unary_expression(p);
    }

    p.close(m, TreeKind::CastExpression);
    p.trace_exit();
}

// type_name
// : specifier_qualifier_list abstract_declarator?
// ;
//
// TypeName = SpecifierQualifierList AbstractDeclarator?
fn type_name(p: &mut Parser) {
    let m = p.open();
    specifier_qualifier_list(p);
    if p.at(TokenKind::IDENTIFIER) {
        abstract_declarator(p);
    }

    p.close(m, TreeKind::TypeName);
}

// specifier_qualifier_list
// : type_specifier specifier_qualifier_list?
// | type_qualifier specifier_qualifier_list?
// ;
//
// SpecifierQualifierList = TypeSpecifier SpecifierQualifierList?
// | TypeQualifier SpecifierQualifierList?
fn specifier_qualifier_list(p: &mut Parser) {
    let m = p.open();
    if p.at(TokenKind::CONST_KW) || p.at(TokenKind::VOLATILE_KW)
    // || p.at(TokenKind::RESTRICT_KW)
    {
        type_qualifier(p);
        specifier_qualifier_list(p);
    } else {
        type_specifier(p);
        if p.at(TokenKind::CONST_KW) || p.at(TokenKind::VOLATILE_KW)
        // || p.at(TokenKind::RESTRICT_KW)
        {
            specifier_qualifier_list(p);
        }
    }

    p.close(m, TreeKind::SpecifierQualifierList);
}

/// **Yacc:**
///
/// ```yacc
/// unary_expression
/// : postfix_expression
/// | INC_OP unary_expression
/// | DEC_OP unary_expression
/// | unary_operator cast_expression
/// | SIZEOF unary_expression
/// | SIZEOF '(' type_name ')'
/// ;
/// ```
///
/// **Ungrammar:**
///
/// ```text
/// UnaryExpression = PostfixExpression
/// | INC_OP UnaryExpression
/// | DEC_OP UnaryExpression
/// | UnaryOperator CastExpression
/// | SIZEOF UnaryExpression
/// | SIZEOF '(' TypeName ')'
/// ```
///
/// **Example C code:**
///
/// ```c
/// // Unary expressions
/// 1;
/// -a;
/// sizeof(a);
/// sizeof(int);
/// sizeof(int *);
/// sizeof(int[10]);
/// ++a;
/// --a;
/// ```
pub(crate) fn unary_expression(p: &mut Parser) {
    p.enter(TreeKind::UnaryExpression);
    let m = p.open();

    if p.at(TokenKind::INC_OP) || p.at(TokenKind::DEC_OP) {
        p.advance();
        unary_expression(p);
    } else if p.at_unary_operator() {
        unary_operator(p);
        cast_expression(p);
    } else if p.at(TokenKind::SIZEOF_KW) {
        p.advance();
        if p.at(TokenKind::LPAREN) {
            p.advance();
            type_name(p);
            p.expect(TokenKind::RPAREN);
        } else {
            unary_expression(p);
        }
    } else {
        // TODO: postfix_expression and error case / reporting
        postfix_expression(p);
    }

    p.close(m, TreeKind::UnaryExpression);
    p.trace_exit();
}

// unary_operator
// : '&'
// | '*'
// | '+'
// | '-'
// | '~'
// | '!'
// ;
//
// UnaryOperator = '&'
// | '*'
// | '+'
// | '-'
// | '~'
// | '!'
fn unary_operator(p: &mut Parser) {
    p.enter(TreeKind::UnaryOperator);
    let m = p.open();
    if p.at_unary_operator() {
        p.advance();
    } else {
        // TODO: error reporting
        p.advance_with_error(&format!(
            "unary operator expected (e.g. '&', '*', '+', '-', '~', '!'). Instead found {:?}",
            p.current(),
        ));
    }

    p.close(m, TreeKind::UnaryOperator);
    p.trace_exit();
}

// postfix_expression
// : primary_expression
// | postfix_expression '[' expression ']'
// | postfix_expression '(' ')'
// | postfix_expression '(' argument_expression_list ')'
// | postfix_expression '.' IDENTIFIER
// | postfix_expression PTR_OP IDENTIFIER
// | postfix_expression INC_OP
// | postfix_expression DEC_OP
// ;
//
// PostfixExpression = PrimaryExpression
// | PostfixExpression '[' Expression ']'
// | PostfixExpression '(' ')'
// | PostfixExpression '(' ArgumentExpressionList ')'
// | PostfixExpression '.' IDENTIFIER
// | PostfixExpression PTR_OP IDENTIFIER
// | PostfixExpression INC_OP
// | PostfixExpression DEC_OP
fn postfix_expression(p: &mut Parser) {
    let m = p.open();
    primary_expression(p);
    while p.at(TokenKind::LBRACKET) ||
        p.at(TokenKind::LPAREN) ||
        p.at(TokenKind::DOT) ||
        p.at(TokenKind::PTR_OP) ||
        p.at(TokenKind::INC_OP) ||
        p.at(TokenKind::DEC_OP)
    {
        if p.at(TokenKind::LBRACKET) {
            p.advance();
            expression(p);
            p.expect(TokenKind::RBRACKET);
        } else if p.at(TokenKind::LPAREN) {
            p.advance();
            if !p.at(TokenKind::RPAREN) {
                argument_expression_list(p);
            }
            p.expect(TokenKind::RPAREN);
        } else if p.at(TokenKind::DOT) || p.at(TokenKind::PTR_OP) {
            p.advance();
            p.expect(TokenKind::IDENTIFIER);
        } else if p.at(TokenKind::INC_OP) || p.at(TokenKind::DEC_OP) {
            p.advance();
        }
    }

    p.close(m, TreeKind::PostfixExpression);
}

// argument_expression_list
// : assignment_expression
// | argument_expression_list ',' assignment_expression
// ;
//
// ArgumentExpressionList = AssignmentExpression
// | ArgumentExpressionList ',' AssignmentExpression
fn argument_expression_list(p: &mut Parser) {
    let m = p.open();
    assignment_expression(p);
    while p.at(TokenKind::COMMA) {
        p.advance();
        assignment_expression(p);
    }

    p.close(m, TreeKind::ArgumentExpressionList);
}

// primary_expression
// 	: IDENTIFIER
// 	| constant
// 	| string
// 	| '(' expression ')'
// 	| generic_selection
// 	;
//
// PrimaryExpression = IDENTIFIER
// | Constant
// | String
// | '(' Expression ')'
// | GenericSelection
pub(crate) fn primary_expression(p: &mut Parser) {
    p.enter(TreeKind::PrimaryExpression);
    let m = p.open();

    println!("primary_expression: {:?} {:?}", p.current_token().kind(), p.current_token().lexeme());

    if p.at(TokenKind::IDENTIFIER) {
        p.advance();
    } else if p.at_constant() {
        constant(p);
    } else if p.at_string() {
        string(p);
    } else if p.at(TokenKind::LPAREN) {
        p.advance();
        expression(p);
        p.expect(TokenKind::RPAREN);
    } else if p.at_generic_selection() {
        generic_selection(p);
    } else {
        // TODO: error reporting
        p.advance_with_error(&format!(
            "primary expression expected (identifier, constant, string, or generic selection). \
             Instead found {:?}",
            p.current()
        ));
    }

    p.close(m, TreeKind::PrimaryExpression);
    p.trace_exit();
}

pub(crate) fn generic_selection(p: &mut Parser) {
    p.enter(TreeKind::GenericSelection);
    let m = p.open();

    p.expect(TokenKind::GENERIC_KW);
    p.expect(TokenKind::LPAREN);
    assignment_expression(p);
    p.expect(TokenKind::COMMA);
    generic_assoc_list(p);
    p.expect(TokenKind::RPAREN);

    p.close(m, TreeKind::GenericSelection);
    p.trace_exit();
}

pub(crate) fn generic_assoc_list(p: &mut Parser) {
    p.enter(TreeKind::GenericAssocList);
    let m = p.open();

    generic_association(p);

    while p.at(TokenKind::COMMA) {
        p.advance();
        generic_association(p);
    }

    p.close(m, TreeKind::GenericAssocList);
    p.trace_exit();
}

// generic_association
// 	: type_name ':' assignment_expression
// 	| DEFAULT ':' assignment_expression
// 	;
//
// GenericAssociation = TypeName ':' AssignmentExpression
// | DEFAULT ':' AssignmentExpression
pub(crate) fn generic_association(p: &mut Parser) {
    p.enter(TreeKind::GenericAssociation);
    let m = p.open();

    if p.at(TokenKind::DEFAULT_KW) {
        p.advance();
        p.expect(TokenKind::COLON);
        assignment_expression(p);
    } else {
        type_name(p);
        p.expect(TokenKind::COLON);
        assignment_expression(p);
    }

    p.close(m, TreeKind::GenericAssociation);
    p.trace_exit();
}

// constant
// : INTEGER_CONSTANT
// | FLOATING_CONSTANT
// | ENUMERATION_CONSTANT
// ;
//
// Constant = INTEGER_CONSTANT
// | FLOATING_CONSTANT
// | ENUMERATION_CONSTANT
pub(crate) fn constant(p: &mut Parser) {
    p.enter(TreeKind::Constant);
    let m = p.open();

    if p.at(TokenKind::INTEGER_CONSTANT) ||
        p.at(TokenKind::FLOATING_CONSTANT) ||
        p.at(TokenKind::IDENTIFIER)
    // ENUMERATION_CONSTANT
    {
        p.advance();
    } else {
        p.advance_with_error(
            "Expected a constant (integer, floating, or enumeration)\nExamples of constants:\n  \
             1\n  1.0\n  ONE",
        );
    }

    p.close(m, TreeKind::Constant);
    p.trace_exit();
}

// string
// : STRING_LITERAL
// | FUNC_NAME
// ;
//
// String = STRING_LITERAL
// | FUNC_NAME
pub(crate) fn string(p: &mut Parser) {
    p.enter(TreeKind::String);
    let m = p.open();

    if p.at(TokenKind::STRING) || p.at(TokenKind::FUNC_NAME_KW) {
        p.advance();
    } else {
        p.advance_with_error(&format!(
            "Expected a string or function name literal (e.g. \"hello\", __func__), but instead
                found {:?}",
            p.current()
        ));
    }

    p.close(m, TreeKind::String);
    p.trace_exit();
}

// parameter_type_list
// : parameter_list (',' ELLIPSIS)?
// ;
//
// ParameterTypeList = ParameterList (',' ELLIPSIS)?
fn parameter_type_list(p: &mut Parser) {
    p.enter(TreeKind::ParameterTypeList);
    let m = p.open();

    parameter_list(p);
    if p.at(TokenKind::COMMA) {
        p.advance();
        p.expect(TokenKind::ELLIPSIS);
    }

    p.close(m, TreeKind::ParameterTypeList);
    p.trace_exit();
}

// parameter_list
// : parameter_declaration (',' parameter_declaration)*
// ;
//
// ParameterList = ParameterDeclaration (',' ParameterDeclaration)*
fn parameter_list(p: &mut Parser) {
    p.enter(TreeKind::ParameterList);
    let m = p.open();

    parameter_declaration(p);
    while p.at(TokenKind::COMMA) {
        p.advance();
        parameter_declaration(p);
    }

    p.close(m, TreeKind::ParameterList);
    p.trace_exit();
}

// parameter_declaration
// : declaration_specifiers declarator
// | declaration_specifiers abstract_declarator?
// ;
//
// ParameterDeclaration = DeclarationSpecifiers Declarator
// | DeclarationSpecifiers (AbstractDeclarator)?
fn parameter_declaration(p: &mut Parser) {
    // Useful for reference:
    //
    // declarator
    // 	: pointer direct_declarator
    // 	| direct_declarator
    // 	;

    // abstract_declarator
    // 	: pointer direct_abstract_declarator
    // 	| pointer
    // 	| direct_abstract_declarator
    // 	;

    p.enter(TreeKind::ParameterDeclaration);
    let m = p.open();

    declaration_specifiers(p);

    if p.at(TokenKind::STAR) {
        // We have a pointer, so could either be one of three cases:
        // 1. pointer direct_declarator
        // 2. pointer abstract_declarator
        // 3. pointer
        // abstract_declarator(p);

        // First, we need to check if we have a pointer or not.
        pointer(p);

        // Now, we need to check if we have a direct_declarator or
        // abstract_declarator.
        if p.at(TokenKind::LPAREN) {
            // We have an abstract_declarator
            direct_abstract_declarator(p);
        } else {
            // We have a direct_declarator
            direct_declarator(p);
        }
    } else {
        direct_declarator(p);
    }

    // abstract_declarator(p);
    // declarator(p);

    p.close(m, TreeKind::ParameterDeclaration);
    p.trace_exit();
}

/// # [Yacc](http://www.quut.com/c/ANSI-C-grammar-y.html#abstract_declarator)
///
/// ```yacc
/// abstract_declarator
/// : pointer
/// | pointer? direct_abstract_declarator
/// ;
/// ```
///
/// # [Ungrammar](https://rust-analyzer.github.io//blog/2020/10/24/introducing-ungrammar.html)
///
/// ```ungrammar
/// AbstractDeclarator = Pointer
/// | (Pointer)? DirectAbstractDeclarator
/// ```
fn abstract_declarator(p: &mut Parser) {
    p.enter(TreeKind::AbstractDeclarator);
    let m = p.open();

    // TODO: make more rigorous (i.e. if we don't see either, emit an error and
    // advance letting the user know we expected either a pointer or
    // direct_abstract_declarator)
    if p.at(TokenKind::STAR) {
        pointer(p);
    }
    if p.at(TokenKind::LPAREN) {
        direct_abstract_declarator(p);
    }

    p.close(m, TreeKind::AbstractDeclarator);
    p.trace_exit();
}

/// # [Yacc](http://www.quut.com/c/ANSI-C-grammar-y.html#direct_abstract_declarator)
///
/// ```yacc
/// direct_abstract_declarator
///  : '(' abstract_declarator ')'
///  | '[' ']'
///  | '[' '*' ']'
///  | '[' STATIC type_qualifier_list assignment_expression ']'
///  | '[' STATIC assignment_expression ']'
///  | '[' type_qualifier_list STATIC assignment_expression ']'
///  | '[' type_qualifier_list assignment_expression ']'
///  | '[' type_qualifier_list ']'
///  | '[' assignment_expression ']'
///  | direct_abstract_declarator '[' ']'
///  | direct_abstract_declarator '[' '*' ']'
///  | direct_abstract_declarator '[' STATIC type_qualifier_list
/// assignment_expression ']'
/// | direct_abstract_declarator '[' STATIC
/// assignment_expression ']'
/// | direct_abstract_declarator '['
/// type_qualifier_list assignment_expression ']'
/// | direct_abstract_declarator
/// '[' type_qualifier_list STATIC assignment_expression ']'
///  | direct_abstract_declarator '[' type_qualifier_list ']'
///  | direct_abstract_declarator '[' assignment_expression ']'
///  | '(' ')'
///  | '(' parameter_type_list ')'
///  | direct_abstract_declarator '(' ')'
///  | direct_abstract_declarator '(' parameter_type_list ')'
///  ;
/// ```
fn direct_abstract_declarator(p: &mut Parser) {
    p.enter(TreeKind::DirectAbstractDeclarator);
    let m = p.open();

    // TODO: implement

    p.close(m, TreeKind::DirectAbstractDeclarator);
    p.trace_exit();
}

// // direct_abstract_declarator
// // : '(' abstract_declarator ')'
// // | '[' ']'
// // | '[' constant_expression? ']'
// // | direct_abstract_declarator '[' ']'
// // | direct_abstract_declarator '[' constant_expression? ']'
// // | '(' ')'
// // | '(' parameter_type_list? ')'
// // | direct_abstract_declarator '(' ')'
// // | direct_abstract_declarator '(' parameter_type_list? ')'
// // ;
// //
// // DirectAbstractDeclarator = '(' AbstractDeclarator ')'
// // | '[' ']'
// // | '[' (ConstantExpression)? ']'
// // | DirectAbstractDeclarator '[' ']'
// // | DirectAbstractDeclarator '[' (ConstantExpression)? ']'
// // | '(' ')'
// // | '(' (ParameterTypeList)? ')'
// // | DirectAbstractDeclarator '(' ')'
// // | DirectAbstractDeclarator '(' (ParameterTypeList)? ')'
// fn direct_abstract_declarator(p: &mut Parser) {
//     p.enter(TreeKind::DirectAbstractDeclarator);
//     let m = p.open();

//     if p.at(TokenKind::LPAREN) {
//         p.advance();
//         abstract_declarator(p);
//         p.expect(TokenKind::RPAREN);
//     } else if p.at(TokenKind::LBRACKET) {
//         p.advance();
//         if !p.at(TokenKind::RBRACKET) {
//             constant_expression(p);
//         }
//         p.expect(TokenKind::RBRACKET);
//     } else {
//         direct_abstract_declarator(p);
//         if p.at(TokenKind::LBRACKET) {
//             p.advance();
//             if !p.at(TokenKind::RBRACKET) {
//                 constant_expression(p);
//             }
//             p.expect(TokenKind::RBRACKET);
//         } else if p.at(TokenKind::LPAREN) {
//             p.advance();
//             if !p.at(TokenKind::RPAREN) {
//                 parameter_type_list(p);
//             }
//             p.expect(TokenKind::RPAREN);
//         } else {
//             p.expect(TokenKind::LPAREN);
//             if !p.at(TokenKind::RPAREN) {
//                 parameter_type_list(p);
//             }
//             p.expect(TokenKind::RPAREN);
//         }
//     }

//     p.close(m, TreeKind::DirectAbstractDeclarator);
// }

// pointer
// : '*'
// | '*' type_qualifier_list
// | '*' pointer
// | '*' type_qualifier_list pointer
// ;
//
// Pointer = '*' Pointer?
// | '*' TypeQualifierList Pointer?
fn pointer(p: &mut Parser) {
    p.enter(TreeKind::Pointer);
    let m = p.open();

    p.expect(TokenKind::STAR);
    if p.at_type_qualifier() {
        type_qualifier_list(p);
    }
    if p.at(TokenKind::STAR) {
        pointer(p);
    }

    p.close(m, TreeKind::Pointer);
    p.trace_exit();
}

// type_qualifier_list
// : type_qualifier
// | type_qualifier_list type_qualifier
// ;
//
// TypeQualifierList = (TypeQualifier)* TypeQualifier
fn type_qualifier_list(p: &mut Parser) {
    p.enter(TreeKind::TypeQualifierList);
    let m = p.open();

    while p.at_type_qualifier() {
        type_qualifier(p);
    }

    p.close(m, TreeKind::TypeQualifierList);
    p.trace_exit();
}

// declaration_list
// : declaration
// | declaration_list declaration
// ;
//
// DeclarationList = (Declaration)* Declaration
fn declaration_list(p: &mut Parser) {
    p.enter(TreeKind::DeclarationList);
    let m = p.open();

    // TODO: make this logic more robust

    while !p.eof() && !p.at(TokenKind::RBRACE) {
        declaration(p);
    }

    p.close(m, TreeKind::DeclarationList);
    p.trace_exit();
}

// init_declarator_list
// : init_declarator
// | init_declarator_list ',' init_declarator
// ;
//
// InitDeclaratorList = (InitDeclarator)* InitDeclarator
fn init_declarator_list(p: &mut Parser) {
    p.enter(TreeKind::InitDeclaratorList);
    let m = p.open();

    init_declarator(p);
    while p.eat(TokenKind::COMMA) {
        init_declarator(p);
    }

    p.close(m, TreeKind::InitDeclaratorList);
    p.trace_exit();
}

// init_declarator
// : declarator
// | declarator '=' initializer
// ;
//
// InitDeclarator = Declarator ('=' Initializer)?
fn init_declarator(p: &mut Parser) {
    p.enter(TreeKind::InitDeclarator);
    let m = p.open();

    declarator(p);
    if p.eat(TokenKind::EQ) {
        initializer(p);
    }

    p.close(m, TreeKind::InitDeclarator);
    p.trace_exit();
}

// initializer
// : assignment_expression
// | '{' initializer_list '}'
// | '{' initializer_list ',' '}'
// ;
//
// Initializer = AssignmentExpression
// | '{' InitializerList '}'
// | '{' InitializerList ',' '}'
fn initializer(p: &mut Parser) {
    p.enter(TreeKind::Initializer);
    let m = p.open();

    if p.at(TokenKind::LBRACE) {
        p.advance();
        initializer_list(p);
        if p.at(TokenKind::COMMA) {
            p.advance();
        }
        p.expect(TokenKind::RBRACE);
    } else {
        assignment_expression(p);
    }

    p.close(m, TreeKind::Initializer);
    p.trace_exit();
}

// initializer_list
// : initializer
// | initializer_list ',' initializer
// ;
//
// InitializerList = (Initializer)* Initializer
fn initializer_list(p: &mut Parser) {
    p.enter(TreeKind::InitializerList);
    let m = p.open();

    initializer(p);
    while p.eat(TokenKind::COMMA) {
        initializer(p);
    }

    p.close(m, TreeKind::InitializerList);
    p.trace_exit();
}

// declaration_specifiers
// 	: storage_class_specifier declaration_specifiers
// 	| storage_class_specifier
// 	| type_specifier declaration_specifiers
// 	| type_specifier
// 	| type_qualifier declaration_specifiers
// 	| type_qualifier
// 	| function_specifier declaration_specifiers
// 	| function_specifier
// 	| alignment_specifier declaration_specifiers
// 	| alignment_specifier
// 	;
//
// DeclarationSpecifiers = StorageClassSpecifier DeclarationSpecifiers?
// | TypeSpecifier DeclarationSpecifiers?
// | TypeQualifier DeclarationSpecifiers?
// | FunctionSpecifier DeclarationSpecifiers?
// | AlignmentSpecifier DeclarationSpecifiers?
fn declaration_specifiers(p: &mut Parser) {
    p.enter(TreeKind::DeclarationSpecifiers);
    let m = p.open();

    println!(
        "declaration_specifiers: {:?} {:?}",
        p.current_token().kind(),
        p.current_token().lexeme()
    );

    if p.at_storage_class_specifier() {
        storage_class_specifier(p);
        if p.at_declaration_specifier() {
            declaration_specifiers(p);
        }
    } else if p.at_type_specifier() {
        type_specifier(p);
        if p.at_declaration_specifier() {
            declaration_specifiers(p);
        }
    } else if p.at_type_qualifier() {
        type_qualifier(p);
        if p.at_declaration_specifier() {
            declaration_specifiers(p);
        }
    } else if p.at_function_specifier() {
        function_specifier(p);
        if p.at_declaration_specifier() {
            declaration_specifiers(p);
        }
    } else if p.at_alignment_specifier() {
        alignment_specifier(p);
        if p.at_declaration_specifier() {
            declaration_specifiers(p);
        }
    } else {
        // TODO: error reporting
        p.advance_with_error(&format!(
            "declaration specifier expected (storage class, type, function or alignment \
             specifier). Instead found {:?} {:?}\n\nHint: If you are trying to declare a \
             variable, make sure you have a type specifier before the variable name. For example, \
             `int x = 0;`.\n\nExamples of declaration specificies are `int`, `char`, `short`,  \
             `float`, `signed`, `const`, `volatile`, `inline`, `noreturn`, `struct`, `union`, \
             `enum`, etc.\n",
            p.current(),
            p.current_token().lexeme()
        ));
    }

    p.close(m, TreeKind::DeclarationSpecifiers);
    p.trace_exit();
}

// function_specifier
// 	: INLINE
// 	| NORETURN
// 	;
//
// FunctionSpecifier = INLINE
// | NORETURN
fn function_specifier(p: &mut Parser) {
    p.enter(TreeKind::FunctionSpecifier);
    let m = p.open();

    if p.at_function_specifier() {
        p.advance();
    } else {
        // TODO: error reporting
        p.advance_with_error(&format!(
            "expected function specifier (`inline` or `noreturn`), but found {}",
            p.nth(0),
        ));
    }

    p.close(m, TreeKind::FunctionSpecifier);
    p.trace_exit();
}

// alignment_specifier
// 	: ALIGNAS '(' type_name ')'
// 	| ALIGNAS '(' constant_expression ')'
// 	;
//
// AlignmentSpecifier = ALIGNAS '(' TypeName ')'
// | ALIGNAS '(' ConstantExpression ')'
fn alignment_specifier(p: &mut Parser) {
    p.enter(TreeKind::AlignmentSpecifier);
    let m = p.open();

    p.expect(TokenKind::ALIGNAS_KW);
    p.expect(TokenKind::LPAREN);

    if p.at_constant_expression() {
        constant_expression(p);
    } else {
        type_name(p);
    }
    p.expect(TokenKind::RPAREN);

    p.close(m, TreeKind::AlignmentSpecifier);
    p.trace_exit();
}

// NOTE: OLD VERSION OF Syntax
// fn declaration_specifiers(p: &mut Parser) {
//   p.enter(TreeKind::DeclarationSpecifiers);
//   let m = p.open();

//   while p.at_any(&[
//     TokenKind::AUTO_KW,
//     TokenKind::REGISTER_KW,
//     TokenKind::STATIC_KW,
//     TokenKind::EXTERN_KW,
//     TokenKind::TYPEDEF_KW,
//     TokenKind::CONST_KW,
//     TokenKind::VOLATILE_KW,
//     TokenKind::VOID_KW,
//     TokenKind::CHAR_KW,
//     TokenKind::SHORT_KW,
//     TokenKind::INT_KW,
//     TokenKind::LONG_KW,
//     // TokenKind::FLOAT,
//     TokenKind::DOUBLE_KW,
//     TokenKind::SIGNED_KW,
//     TokenKind::UNSIGNED_KW,
//     TokenKind::STRUCT_KW,
//     TokenKind::UNION_KW,
//     TokenKind::ENUM_KW,
//   ]) {
//     if p.at_any(&[
//       TokenKind::AUTO_KW,
//       TokenKind::REGISTER_KW,
//       TokenKind::STATIC_KW,
//       TokenKind::EXTERN_KW,
//       TokenKind::TYPEDEF_KW,
//     ]) {
//       storage_class_specifier(p);
//     } else if p.at_any(&[
//       TokenKind::VOID_KW,
//       TokenKind::CHAR_KW,
//       TokenKind::SHORT_KW,
//       TokenKind::INT_KW,
//       TokenKind::LONG_KW,
//       TokenKind::FLOAT_KW,
//       TokenKind::DOUBLE_KW,
//       TokenKind::SIGNED_KW,
//       TokenKind::UNSIGNED_KW,
//       TokenKind::STRUCT_KW,
//       TokenKind::UNION_KW,
//       TokenKind::ENUM_KW,
//     ]) {
//       type_specifier(p);
//     } else if p
//       .at_any(&[TokenKind::CONST_KW, TokenKind::VOLATILE_KW])
//     {
//       type_qualifier(p);
//     }
//   }

//   p.close(m, TreeKind::DeclarationSpecifiers);
//   p.trace_exit();
// }

// storage_class_specifier
// : AUTO_KW
// | REGISTER_KW
// | STATIC_KW
// | EXTERN_KW
// | TYPEDEF_KW
// ;
//
// StorageClassSpecifier = Auto | Register | Static | Extern | Typedef
fn storage_class_specifier(p: &mut Parser) {
    p.enter(TreeKind::StorageClassSpecifier);
    let m = p.open();

    if p.at_any(&[
        TokenKind::AUTO_KW,
        TokenKind::REGISTER_KW,
        TokenKind::STATIC_KW,
        TokenKind::EXTERN_KW,
        TokenKind::TYPEDEF_KW,
    ]) {
        p.advance();
    } else {
        // TODO: error reporting
        // p.error("expected storage class specifier");
        p.advance_with_error(&format!("expected storage class specifier, but found {}", p.nth(0),));
    }

    p.close(m, TreeKind::StorageClassSpecifier);
    p.trace_exit();
}

// type_specifier
// : VOID_KW
// | CHAR_KW
// | SHORT_KW
// | INT_KW
// | LONG_KW
// | FLOAT_KW
// | DOUBLE_KW
// | SIGNED_KW
// | UNSIGNED_KW
// | struct_or_union_specifier
// | enum_specifier
// | TYPE_NAME
// ;
//
// TypeSpecifier = Void | Char | Short | Int | Long | Float | Double | Signed |
// Unsigned | StructOrUnionSpecifier | EnumSpecifier | TypeName
fn type_specifier(p: &mut Parser) {
    p.enter(TreeKind::TypeSpecifier);
    let m = p.open();

    if p.at_any(&[
        TokenKind::VOID_KW,
        TokenKind::CHAR_KW,
        TokenKind::SHORT_KW,
        TokenKind::INT_KW,
        TokenKind::LONG_KW,
        TokenKind::FLOAT_KW,
        TokenKind::DOUBLE_KW,
        TokenKind::SIGNED_KW,
        TokenKind::UNSIGNED_KW,
    ]) {
        p.advance();
    } else if p.at_any(&[TokenKind::STRUCT_KW, TokenKind::UNION_KW]) {
        struct_or_union_specifier(p);
    } else if p.at(TokenKind::ENUM_KW) {
        enum_specifier(p);
    } else if p.at(TokenKind::IDENTIFIER) {
        p.advance();
    } else {
        // TODO: Error reporting.
    }

    p.close(m, TreeKind::TypeSpecifier);
}

// enum_specifier
// : ENUM '{' enumerator_list '}'
// | ENUM IDENTIFIER '{' enumerator_list '}'
// | ENUM IDENTIFIER
// ;
//
// EnumSpecifier = Enum LBrace EnumeratorList RBrace | Enum Ident LBrace
// EnumeratorList RBrace | Enum Ident
fn enum_specifier(p: &mut Parser) {
    p.enter(TreeKind::EnumSpecifier);
    let m = p.open();

    if p.at(TokenKind::ENUM_KW) {
        p.advance();
    } else {
        // TODO: Error reporting.
    }

    if p.at(TokenKind::IDENTIFIER) {
        p.advance();
    }

    if p.at(TokenKind::LBRACE) {
        p.advance();
        enumerator_list(p);
        if p.at(TokenKind::RBRACE) {
            p.advance();
        } else {
            // TODO: Error reporting.
        }
    }

    p.close(m, TreeKind::EnumSpecifier);
}

// enumerator_list
// : enumerator (',' enumerator)*
// ;
//
// EnumeratorList = Enumerator (Comma Enumerator)*
fn enumerator_list(p: &mut Parser) {
    p.enter(TreeKind::EnumeratorList);
    let m = p.open();

    enumerator(p);

    while p.at(TokenKind::COMMA) {
        p.advance();
        enumerator(p);
    }

    p.close(m, TreeKind::EnumeratorList);
}

// enumerator
// : IDENTIFIER ('=' constant_expression)?
// ;
//
// Enumerator = Ident (Assign ConstantExpression)?
fn enumerator(p: &mut Parser) {
    p.enter(TreeKind::Enumerator);
    let m = p.open();

    if p.at(TokenKind::IDENTIFIER) {
        p.advance();
    } else {
        // TODO: Error reporting.
    }

    if p.at(TokenKind::EQ) {
        p.advance();
        constant_expression(p);
    }

    p.close(m, TreeKind::Enumerator);
}

// struct_or_union_specifier
// 	: struct_or_union '{' struct_declaration_list '}'
// 	| struct_or_union IDENTIFIER '{' struct_declaration_list '}'
// 	| struct_or_union IDENTIFIER
// 	;
//
// StructOrUnionSpecifier = StructOrUnion LBrace StructDeclarationList RBrace
fn struct_or_union_specifier(p: &mut Parser) {
    p.enter(TreeKind::StructOrUnionSpecifier);
    let m = p.open();

    struct_or_union(p);

    if p.at(TokenKind::IDENTIFIER) {
        p.advance();
    }

    if p.at(TokenKind::LBRACE) {
        p.advance();
        struct_declaration_list(p);
        p.expect(TokenKind::RBRACE);
    } else {
        p.advance_with_error(&format!(
            "expected struct or union specifier, but instead found {}",
            p.nth(0),
        ));
    }

    p.close(m, TreeKind::StructOrUnionSpecifier);
    p.trace_exit();
}

// struct_or_union
// : STRUCT_KW
// | UNION_KW
// ;
fn struct_or_union(p: &mut Parser) {
    p.enter(TreeKind::StructOrUnion);
    let m = p.open();

    if p.at_any(&[TokenKind::STRUCT_KW, TokenKind::UNION_KW]) {
        p.advance();
    } else {
        // TODO: Error reporting.
        p.advance_with_error(&format!(
            "expected struct or union specifier, but instead found {}",
            p.nth(0),
        ));
    }

    p.close(m, TreeKind::StructOrUnion);
    p.trace_exit();
}

// FIXME: OLD
//  p.enter(TreeKind::StructOrUnionSpecifier);
//     let m = p.open();

//     if p.at(TokenKind::STRUCT_KW) | p.at(TokenKind::UNION_KW) {
//         p.advance();
//     } else {
//         // TODO: Error reporting.
//     }

//     if p.at(TokenKind::IDENTIFIER) {
//         p.advance();
//     }

//     if p.at(TokenKind::LBRACE) {
//         p.advance();
//         struct_declaration_list(p);
//         p.expect(TokenKind::RBRACE);
//     }

//     p.close(m, TreeKind::StructOrUnionSpecifier);

// struct_declaration_list
// : struct_declaration
// | struct_declaration_list struct_declaration
// ;
//
// StructDeclarationList = StructDeclaration | StructDeclarationList
// StructDeclaration
fn struct_declaration_list(p: &mut Parser) {
    p.enter(TreeKind::StructDeclarationList);
    let m = p.open();

    struct_declaration(p);

    while p.at_any(&[
        TokenKind::CHAR_KW,
        TokenKind::SHORT_KW,
        TokenKind::INT_KW,
        TokenKind::LONG_KW,
        TokenKind::FLOAT_KW,
        TokenKind::DOUBLE_KW,
        TokenKind::SIGNED_KW,
        TokenKind::UNSIGNED_KW,
        TokenKind::STRUCT_KW,
        TokenKind::UNION_KW,
        TokenKind::ENUM_KW,
        TokenKind::CONST_KW,
        TokenKind::VOLATILE_KW,
        TokenKind::IDENTIFIER,
    ]) {
        struct_declaration(p);
    }

    p.close(m, TreeKind::StructDeclarationList);
}

// struct_declaration
// : specifier_qualifier_list struct_declarator_list? SEMICOLON
// ;
//
// StructDeclaration = SpecifierQualifierList StructDeclaratorList? Semicolon
fn struct_declaration(p: &mut Parser) {
    p.enter(TreeKind::StructDeclaration);
    let m = p.open();

    specifier_qualifier_list(p);

    if p.at_any(&[TokenKind::IDENTIFIER, TokenKind::STAR, TokenKind::LPAREN]) {
        struct_declarator_list(p);
    }

    p.expect(TokenKind::SEMICOLON);

    p.close(m, TreeKind::StructDeclaration);
}

// struct_declarator_list
// : struct_declarator
// | struct_declarator_list COMMA struct_declarator
// ;
//
// StructDeclaratorList = StructDeclarator | StructDeclaratorList Comma
// StructDeclarator
fn struct_declarator_list(p: &mut Parser) {
    p.enter(TreeKind::StructDeclaratorList);
    let m = p.open();

    struct_declarator(p);

    while p.at(TokenKind::COMMA) {
        p.advance();
        struct_declarator(p);
    }

    p.close(m, TreeKind::StructDeclaratorList);
}

// struct_declarator
// : declarator
// | COLON constant_expression
// | declarator COLON constant_expression
// ;
//
// StructDeclarator = Declarator | Colon ConstantExpression | Declarator Colon
// ConstantExpression
fn struct_declarator(p: &mut Parser) {
    p.enter(TreeKind::StructDeclarator);
    let m = p.open();

    if p.at(TokenKind::COLON) {
        p.advance();
        constant_expression(p);
    } else {
        declarator(p);

        if p.at(TokenKind::COLON) {
            p.advance();
            constant_expression(p);
        }
    }

    p.close(m, TreeKind::StructDeclarator);
}

// type_qualifier
// : CONST_KW
// | VOLATILE_KW
// | RESTRICT_KW
// | ATOMIC_KW
// ;
//
// TypeQualifier = Const | Volatile | Restrict | Atomic
fn type_qualifier(p: &mut Parser) {
    let m = p.open();

    if p.at_type_qualifier() {
        p.advance();
    } else {
        // TODO: Error reporting.
        p.error("expected type qualifier (const, volatile, restrict, or atomic)");
    }

    p.close(m, TreeKind::TypeQualifier);
}

// Current log behavior:
// DEBUG rcc::parser: PARSER (TYPEDEF_KW, 'typedef', 0..7) -> translation_unit
// DEBUG rcc::parser: PARSER (TYPEDEF_KW, 'typedef', 0..7) ->
// external_declaration
//
// Desired log behavior: (Includes call stack information)
// DEBUG rcc::parser: PARSER TYPEDEF_KW 0..7 -> translation_unit
// DEBUG rcc::parser: PARSER TYPEDEF_KW 0..7 -> translation_unit
//                                           | -> external_declaration
