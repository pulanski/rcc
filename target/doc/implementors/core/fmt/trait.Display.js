(function () {
    var implementors = {
        rcc: [
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" title="trait core::fmt::Display">Display</a> for <a class="enum" href="rcc/ast/enum.TreeKind.html" title="enum rcc::ast::TreeKind">TreeKind</a>',
            ],
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" title="trait core::fmt::Display">Display</a> for <a class="struct" href="rcc/ast/struct.Tree.html" title="struct rcc::ast::Tree">Tree</a>',
            ],
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" title="trait core::fmt::Display">Display</a> for <a class="enum" href="rcc/lexer/enum.TokenKind.html" title="enum rcc::lexer::TokenKind">TokenKind</a>',
            ],
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" title="trait core::fmt::Display">Display</a> for <a class="struct" href="rcc/lexer/struct.Span.html" title="struct rcc::lexer::Span">Span</a>',
            ],
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" title="trait core::fmt::Display">Display</a> for <a class="struct" href="rcc/lexer/struct.Token.html" title="struct rcc::lexer::Token">Token</a>',
            ],
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" title="trait core::fmt::Display">Display</a> for <a class="enum" href="rcc/preprocessor/gpp/enum.Error.html" title="enum rcc::preprocessor::gpp::Error">Error</a>',
            ],
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" title="trait core::fmt::Display">Display</a> for <a class="enum" href="rcc/preprocessor/enum.Token.html" title="enum rcc::preprocessor::Token">Token</a>',
            ],
        ],
        resilient_ll: [
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" title="trait core::fmt::Display">Display</a> for <a class="enum" href="resilient_ll/enum.TokenKind.html" title="enum resilient_ll::TokenKind">TokenKind</a>',
            ],
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" title="trait core::fmt::Display">Display</a> for <a class="struct" href="resilient_ll/struct.Token.html" title="struct resilient_ll::Token">Token</a>',
            ],
        ],
    };
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})();
