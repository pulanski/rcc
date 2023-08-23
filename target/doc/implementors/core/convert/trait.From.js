(function () {
    var implementors = {
        rcc: [
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" title="trait core::convert::From">From</a>&lt;<a class="struct" href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" title="struct core::ops::range::Range">Range</a>&lt;<a class="primitive" href="https://doc.rust-lang.org/nightly/std/primitive.usize.html">usize</a>&gt;&gt; for <a class="struct" href="rcc/lexer/struct.Span.html" title="struct rcc::lexer::Span">Span</a>',
            ],
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" title="trait core::convert::From">From</a>&lt;<a class="struct" href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" title="struct std::io::error::Error">Error</a>&gt; for <a class="enum" href="rcc/preprocessor/gpp/enum.Error.html" title="enum rcc::preprocessor::gpp::Error">Error</a>',
            ],
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" title="trait core::convert::From">From</a>&lt;<a class="struct" href="https://doc.rust-lang.org/nightly/alloc/string/struct.FromUtf8Error.html" title="struct alloc::string::FromUtf8Error">FromUtf8Error</a>&gt; for <a class="enum" href="rcc/preprocessor/gpp/enum.Error.html" title="enum rcc::preprocessor::gpp::Error">Error</a>',
            ],
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" title="trait core::convert::From">From</a>&lt;<a class="struct" href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" title="struct alloc::vec::Vec">Vec</a>&lt;<a class="enum" href="rcc/lexer/enum.TokenKind.html" title="enum rcc::lexer::TokenKind">TokenKind</a>, <a class="struct" href="https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html" title="struct alloc::alloc::Global">Global</a>&gt;&gt; for <a class="struct" href="rcc/token_set/struct.TokenSet.html" title="struct rcc::token_set::TokenSet">TokenSet</a>',
            ],
        ],
    };
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})();
