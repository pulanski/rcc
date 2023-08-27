mod ast;
mod cst;
mod diagnostics;
mod lexer;
mod parser;
mod preprocessor;
mod token_set;

use anyhow::Result;
use codespan_reporting::files::SimpleFiles;
use std::{
    fs,
    process::ExitCode,
};
use tracing_subscriber::{
    fmt::Subscriber,
    EnvFilter,
};

// TODO: Get a stats on how many tokens are lexed (for reporting of
// `lexed x tokens at y tokens per second`)
//
// TODO: Get stats on how long each phase of the compiler takes
// then display them in a table at the end of compilation with
// a breakdown of the time spent in each phase
// Maybe use a `--stats` flag to display the stats at the end of compilation
// (it's on by default and can be modified by a key change in the
// `.rcc/config.toml` file) (e.g. lexing, parsing, semantic analysis, code
// generation, etc.) TODO: Add a `--time` flag to display the time spent in each
// phase TODO: Add a `--topics` flag to display and filter the topics that are
// being traced (e.g. `--topics=lex,parse,sem,codegen`)

fn main() -> Result<ExitCode> {
    let subscriber = Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .with_ansi(true)
        .with_max_level(tracing::Level::TRACE)
        .with_line_number(false)
        .with_thread_names(false)
        .without_time() // turn off timestamps
        .finish();

    // Set the subscriber as the default.
    tracing::subscriber::set_global_default(subscriber).expect("failed to set subscriber");

    let mut diagnostics = diagnostics::DiagnosticsEngine::new();

    let file_path = "testdata/parse/ok/medium/recursive.c";
    let text = fs::read_to_string(file_path)?;
    let file_id = diagnostics.add_file(file_path, text);

    // Parse the file into a CST
    let mut cst = parser::parse_file_with_diagnotics(file_path, &mut diagnostics)?;
    // let mut cst = parser::parse_file("testdata/parse/b.c")?;
    // // Reduce the CST to an AST
    let ast = ast::lower_with_diagnostics(file_id, cst.clone(), &mut diagnostics);
    // println!("{:#?}", ast);

    let ast = ast::reduce_with_diagnostics(&mut cst, &mut diagnostics);
    // let file_id = SimpleFiles::new().add("testdata/parse/b.c", cst.source());

    // let ast_sink = ast::lower_with_diagnostics(cst, file_id, &mut
    // diagnostics_engine);

    //
    // println!("{ast:#?}");

    // Recursively parse all files in a directory
    // if let Ok(results) = parser::parse_directory("testdata/parse") {
    //     for result in results {
    //         // println!("{}", result);
    //     }
    // }

    // // Parse the current working directory
    // if let Ok(results) = parser::parse_cwd() {
    //     for result in results {
    //         println!("{}", result);
    //     }
    // }

    Ok(ExitCode::SUCCESS)
}
