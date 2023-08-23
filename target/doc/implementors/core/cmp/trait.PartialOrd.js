(function () {
    var implementors = {
        rcc: [
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" title="trait core::cmp::PartialOrd">PartialOrd</a>&lt;<a class="enum" href="rcc/cst/enum.SyntaxKind.html" title="enum rcc::cst::SyntaxKind">SyntaxKind</a>&gt; for <a class="enum" href="rcc/cst/enum.SyntaxKind.html" title="enum rcc::cst::SyntaxKind">SyntaxKind</a>',
            ],
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" title="trait core::cmp::PartialOrd">PartialOrd</a>&lt;<a class="enum" href="rcc/lexer/enum.TokenKind.html" title="enum rcc::lexer::TokenKind">TokenKind</a>&gt; for <a class="enum" href="rcc/lexer/enum.TokenKind.html" title="enum rcc::lexer::TokenKind">TokenKind</a>',
            ],
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" title="trait core::cmp::PartialOrd">PartialOrd</a>&lt;<a class="struct" href="rcc/lexer/struct.Span.html" title="struct rcc::lexer::Span">Span</a>&gt; for <a class="struct" href="rcc/lexer/struct.Span.html" title="struct rcc::lexer::Span">Span</a>',
            ],
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" title="trait core::cmp::PartialOrd">PartialOrd</a>&lt;<a class="struct" href="rcc/lexer/struct.Token.html" title="struct rcc::lexer::Token">Token</a>&gt; for <a class="struct" href="rcc/lexer/struct.Token.html" title="struct rcc::lexer::Token">Token</a>',
            ],
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" title="trait core::cmp::PartialOrd">PartialOrd</a>&lt;<a class="struct" href="rcc/lexer/struct.TokenStream.html" title="struct rcc::lexer::TokenStream">TokenStream</a>&gt; for <a class="struct" href="rcc/lexer/struct.TokenStream.html" title="struct rcc::lexer::TokenStream">TokenStream</a>',
            ],
        ],
    };
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})();
