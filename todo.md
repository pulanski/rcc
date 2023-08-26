# Language Features to Implement

# Parsing Routines for Grammar

1. [x] Define token types using Rust enums.
2. [x] Implement the lexer to tokenize the input source code.
3. [x] Set up the parser generator (Yacc/Bison) with the provided grammar rules.
4. [x] Create the parsing logic for each production rule in Rust.

## Grammar Productions:

### primary_expression

-   [x] Handle IDENTIFIER
-   [x] Handle constant
-   [x] Handle string
-   [x] Handle '(' expression ')'
-   [x] Handle generic_selection

### constant

-   [x] Handle I_CONSTANT
-   [x] Handle F_CONSTANT
-   [x] Handle ENUMERATION_CONSTANT

### enumeration_constant

-   [x] Handle IDENTIFIER

### string

-   [x] Handle STRING_LITERAL
-   [x] Handle FUNC_NAME

### generic_selection

-   [x] Handle GENERIC '(' assignment_expression ',' generic_assoc_list ')'

### generic_assoc_list

-   [x] Handle generic_association
-   [x] Handle generic_assoc_list ',' generic_association

### generic_association

-   [x] Handle type_name ':' assignment_expression
-   [x] Handle DEFAULT ':' assignment_expression

### postfix_expression

-   [x] Handle primary_expression
-   [x] Handle postfix_expression '[' expression ']'
-   [x] Handle postfix_expression '(' ')'
-   [x] Handle postfix_expression '(' argument_expression_list ')'
-   [x] Handle postfix_expression '.' IDENTIFIER
-   [x] Handle postfix_expression PTR_OP IDENTIFIER
-   [x] Handle postfix_expression INC_OP
-   [x] Handle postfix_expression DEC_OP
-   [x] Handle '(' type_name ')' '{' initializer_list '}'
-   [x] Handle '(' type_name ')' '{' initializer_list ',' '}'

### argument_expression_list

-   [x] Handle assignment_expression
-   [x] Handle argument_expression_list ',' assignment_expression

### unary_expression

-   [x] Handle postfix_expression
-   [x] Handle INC_OP unary_expression
-   [x] Handle DEC_OP unary_expression
-   [x] Handle unary_operator cast_expression
-   [x] Handle SIZEOF unary_expression
-   [x] Handle SIZEOF '(' type_name ')'
-   [x] Handle ALIGNOF '(' type_name ')'

### unary_operator

-   [x] Handle '&'
-   [x] Handle '\*'
-   [x] Handle '+'
-   [x] Handle '-'
-   [x] Handle '~'
-   [x] Handle '!'

### cast_expression

-   [x] Handle unary_expression
-   [x] Handle '(' type_name ')' cast_expression

### multiplicative_expression

-   [x] Handle cast_expression
-   [x] Handle multiplicative_expression '\*' cast_expression
-   [x] Handle multiplicative_expression '/' cast_expression
-   [x] Handle multiplicative_expression '%' cast_expression

### additive_expression

-   [x] Handle multiplicative_expression
-   [x] Handle additive_expression '+' multiplicative_expression
-   [x] Handle additive_expression '-' multiplicative_expression

### ... (Continue this pattern for all other productions)

# AST Nodes for Grammar

1. [ ] Define Rust structs for each type of AST node based on the grammar rules.
2. [ ] Implement conversion functions to create AST nodes during parsing.

## AST Node Structs:

-   [ ] `PrimaryExpression`
-   [ ] `Constant`
-   [ ] `EnumerationConstant`
-   [ ] `String`
-   [ ] `GenericSelection`
-   [ ] `PostfixExpression`
-   [ ] `ArgumentExpressionList`
-   [ ] `UnaryExpression`
-   [ ] `UnaryOperator`
-   [ ] `CastExpression`
-   [ ] `MultiplicativeExpression`
-   [ ] `AdditiveExpression`
-   [ ] ... (Continue this pattern for all other AST nodes)

## Implementing AST Creation:

1. [ ] Create a new AST node struct for each production rule.
2. [ ] Implement conversion functions in the parser to create AST nodes during parsing actions.
3. [ ] Build the AST by composing nodes as the parsing progresses.

Creating AST nodes allows you to represent the parsed input in a structured form that's easier to work with and analyze. Each AST node should correspond to a specific production rule in the grammar.

Remember that both the parser and the AST building process can get quite intricate, and the specifics will depend on the language you're using and the parsing library or tool you're working with.
