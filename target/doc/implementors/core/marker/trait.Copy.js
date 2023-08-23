(function () {
    var implementors = {
        rcc: [
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="enum" href="rcc/cst/enum.SyntaxKind.html" title="enum rcc::cst::SyntaxKind">SyntaxKind</a>',
            ],
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="enum" href="rcc/lexer/enum.TokenKind.html" title="enum rcc::lexer::TokenKind">TokenKind</a>',
            ],
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="struct" href="rcc/lexer/struct.Span.html" title="struct rcc::lexer::Span">Span</a>',
            ],
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="struct" href="rcc/parser/struct.MarkOpened.html" title="struct rcc::parser::MarkOpened">MarkOpened</a>',
            ],
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="struct" href="rcc/preprocessor/gpp/struct.Command.html" title="struct rcc::preprocessor::gpp::Command">Command</a>',
            ],
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="struct" href="rcc/token_set/struct.TokenSet.html" title="struct rcc::token_set::TokenSet">TokenSet</a>',
            ],
        ],
        resilient_ll: [
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="enum" href="resilient_ll/enum.TokenKind.html" title="enum resilient_ll::TokenKind">TokenKind</a>',
            ],
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="struct" href="resilient_ll/struct.MarkOpened.html" title="struct resilient_ll::MarkOpened">MarkOpened</a>',
            ],
        ],
    };
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})();
