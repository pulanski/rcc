# Lowering CST to AST

**NOTE**: It's important to note that these ideas have a few assumptions baked in based on the implementation. For one, we assume that the CST is valid when a lowering is performed, we do emit errors/warnings in code paths where lowering is not possible, but in general we assume that in a e2e compiler pass, if the "--check" pass has failed with invalid syntax, that implies the compilation was preemptively aborted, and we would never reach this transformation step. We also assume that the CST is not mutated after lowering. This is because the lowering process is destructive. It mutates the CST in place. If the CST is mutated after lowering, the AST will be invalid. This is not a problem for us because we don't mutate the CST after lowering. We only mutate the AST after lowering.

- _Simple Algorithm_:

We traverse the **CST** emitted from the parser and try to **reduce/condense** it into an **AST**. We do this by
looking at the current node and its children. If the current node is a
non-terminal, we look at its children and try to reduce them into a single
node. If the current node is a terminal, we just return it as an AST node.
An important note. If the node is a non-terminal and only has one child, we
don't return the child. We return the child's children. This is because we
want to reduce the number of nodes in the AST. For example, if we have a
non-terminal `additive_expression` with only one child `multiplicative_expression`,
we don't want to return an AST node for `additive_expression` with only one
child `multiplicative_expression`. We want to return an AST node for
`multiplicative_expression` instead.


>**NOTE**: Wherever possible, we try to remove tokens from the AST which are not needed for semantic analysis or optimization. However, in general, we try to reduce, without losing any information.
>
> Example: We don't need to keep the `COMMA` tokens in the AST. We can just keep the `IDENTIFIER` tokens and remove the `COMMA` tokens. This is because we can infer the `COMMA` tokens from the `IDENTIFIER` tokens. Or we don't need the `LBRACE` and `RBRACE` tokens in the AST for many concepts.
>
> In a simple case as with structs, we can remove the `STRUCT`, `LBRACE`, and `RBRACE` tokens from the AST and just use natural data structures to model the representation of the data types (i.e. `Struct { fields: Vec<Field> }` instead of `Struct { struct_token: Token, lbrace_token: Token, fields: Vec<Field>, rbrace_token: Token }`).

This lowering process has a few benefits:

1.  It reduces the number of nodes in the AST. This makes it easier to
    traverse and perform semantic analysis on. In general, reducing the complexity of a data structure is a good thing.

2.  It makes the AST more concise. This makes it easier to read and understand, both for humans and machines.

3.  It makes the AST more uniform. This makes it easier to write code that
    operates on the AST, as the general structure becomes more predictable.

## Algorithm in Pseudocode:

```rust
// This is the main entry point for the lowering process,
// transforming a CST into an AST.
fn reduce(node: Node) -> Node {
    // Base case: If the node is a terminal, we just return it.
    if node.is_terminal() {
        return node;
    }

    // If the node is a non-terminal with only one child,
    // we keep reducing until we get to a node with more than one child.
    if node.children.len() == 1 {
        return reduce(node.children[0]);
    }

    // If the node is a non-terminal with more than one child,
    // we reduce each child and return a new node with the reduced children.
    let mut children = Vec::new();
    for child in node.children {
        children.push(reduce(child));
    }

    // Finally, we return a new node with the reduced children.
    return Node {
        kind: node.kind,
        children: children,
    };
}
```

## Example:

Take the following C code:

```c
int x, y, z = 3;
```

The associated CST for this code is:

```txt
└─ExternDecl@2028..2044
    └─Declaration@2028..2044
      └─DeclarationSpecifiers@2028..2031
        └─TypeSpecifier@2028..2031
          \- INT_KW@2028..2031 'int'
      └─InitDeclaratorList@2032..2043
        └─InitDeclarator@2032..2033
          └─Declarator@2032..2033
            └─DirectDeclarator@2032..2033
              \- IDENTIFIER@2032..2033 'x'
        \- COMMA@2033..2034 ','
        └─InitDeclarator@2035..2036
          └─Declarator@2035..2036
            └─DirectDeclarator@2035..2036
              \- IDENTIFIER@2035..2036 'y'
        \- COMMA@2036..2037 ','
        └─InitDeclarator@2038..2043
          └─Declarator@2038..2039
            └─DirectDeclarator@2038..2039
              \- IDENTIFIER@2038..2039 'z'
          \- EQ@2040..2041 '='
          └─Initializer@2042..2043
            └─AssignmentExpression@2042..2043
              └─ConditionalExpression@2042..2043
                └─LogicalOrExpression@2042..2043
                  └─LogicalAndExpression@2042..2043
                    └─InclusiveOrExpression@2042..2043
                      └─ExclusiveOrExpression@2042..2043
                        └─AndExpression@2042..2043
                          └─EqualityExpression@2042..2043
                            └─RelationalExpression@2042..2043
                              └─ShiftExpression@2042..2043
                                └─AdditiveExpression@2042..2043
                                  └─MultiplicativeExpression@2042..2043
                                    └─CastExpression@2042..2043
                                      └─UnaryExpression@2042..2043
                                        └─PostfixExpression@2042..2043
                                          └─PrimaryExpression@2042..2043
                                            └─Constant@2042..2043
                                              \- INTEGER_CONSTANT@2042..2043 '3'
      \- SEMICOLON@2043..2044 ';'
```

While this is a concise representation of the code, it's not very easy to work with. A lot of the information is redundant and not needed for further analysis or optimization. Hence, we can reduce this to the following AST:****

```txt
Declaration@2028..2044
├─TypeSpecifier@2028..2031
│ \-INT_KW@2028..2031 'int'
└─InitDeclaratorList@2032..2043
  ├─DirectDeclarator@2032..2033
  │ \-IDENTIFIER@2032..2033 'x'
  ├─DirectDeclarator@2035..2036
  │ \-IDENTIFIER@2035..2036 'y'
  ├─DirectDeclarator@2038..2039
  │ \-IDENTIFIER@2038..2039 'z'
    \- EQ@2040..2041 '='
  │ ├─Constant@2042..2043
  │ │  \-INTEGER_CONSTANT@2042..2043 '3'
```

or in a more tree-like format:

```digraph
digraph {
    node [shape=record, fontname=monospace];
    edge [fontname=monospace];
    0 [label="Declaration@2028..2044"];
    1 [label="TypeSpecifier@2028..2031"];
    2 [label="INT_KW@2028..2031 'int'"];
    3 [label="InitDeclaratorList@2032..2043"];
    4 [label="DirectDeclarator@2032..2033"];
    5 [label="IDENTIFIER@2032..2033 'x'"];
    6 [label="DirectDeclarator@2035..2036"];
    7 [label="IDENTIFIER@2035..2036 'y'"];
    8 [label="DirectDeclarator@2038..2039"];
    9 [label="IDENTIFIER@2038..2039 'z'"];
    10 [label="EQ@2040..2041 '='"];
    11 [label="Constant@2042..2043"];
    12 [label="INTEGER_CONSTANT@2042..2043 '3'"];
    0 -> 1;
    1 -> 2;
    0 -> 3;
    3 -> 4;
    4 -> 5;
    3 -> 6;
    6 -> 7;
    3 -> 8;
    8 -> 9;
    8 -> 10;
    10 -> 11;
    11 -> 12;
}
```
