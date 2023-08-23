(function () {
    var implementors = {
        rcc: [
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" title="trait core::default::Default">Default</a> for <a class="struct" href="rcc/lexer/struct.Span.html" title="struct rcc::lexer::Span">Span</a>',
            ],
            [
                'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" title="trait core::default::Default">Default</a> for <a class="struct" href="rcc/preprocessor/gpp/struct.Context.html" title="struct rcc::preprocessor::gpp::Context">Context</a>',
            ],
        ],
    };
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})();
