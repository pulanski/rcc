(function () {
    var implementors = {
        rcc: [
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" title="trait core::cmp::Ord">Ord</a> for <a class="enum" href="rcc/cst/enum.SyntaxKind.html" title="enum rcc::cst::SyntaxKind">SyntaxKind</a>',
            ],
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" title="trait core::cmp::Ord">Ord</a> for <a class="enum" href="rcc/lexer/enum.TokenKind.html" title="enum rcc::lexer::TokenKind">TokenKind</a>',
            ],
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" title="trait core::cmp::Ord">Ord</a> for <a class="struct" href="rcc/lexer/struct.Span.html" title="struct rcc::lexer::Span">Span</a>',
            ],
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" title="trait core::cmp::Ord">Ord</a> for <a class="struct" href="rcc/lexer/struct.Token.html" title="struct rcc::lexer::Token">Token</a>',
            ],
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" title="trait core::cmp::Ord">Ord</a> for <a class="struct" href="rcc/lexer/struct.TokenStream.html" title="struct rcc::lexer::TokenStream">TokenStream</a>',
            ],
        ],
    };
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})();
