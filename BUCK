# A list of available rules and their signatures can be found here: https://buck2.build/docs/api/rules/

genrule(
    name = "hello_world",
    out = "out.txt",
    cmd = "echo BUILT BY BUCK2> $OUT",
)

alias(
    name = "rcc",
    actual = "//crates/rcc:rcc",
)

alias(
    name = "resilient_ll",
    actual = "//crates/resilient_ll:resilient_ll",
)
