use std::{
    path::{Path, PathBuf},
    process::exit,
};

use anyhow::{anyhow, Context, Result};
use chimera_text::core::log::initialize_logging;
use clap::{Args, Parser, Subcommand};
use log::LevelFilter;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    /// overrides the log level
    #[arg(short = 'l', long, default_value = "WARN", verbatim_doc_comment)]
    log_level: LevelFilter,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// merges two ebooks together to create a parallel text
    Merge(Merge),
    /// command for testing
    #[cfg(debug_assertions)]
    Debug,
}

#[derive(Args, Debug)]
struct Merge {
    /// the first text to use as input
    /// this text's title, chapter names, cover image, etc will be copied to the merged text
    /// this text's contents will show up first when alternating between parallel texts
    #[arg(short = 'a', long, verbatim_doc_comment)]
    text_a: PathBuf,
    /// the second text to use as input
    /// this text's contents will show up second when alternating between parallel texts
    #[arg(short = 'b', long, verbatim_doc_comment)]
    text_b: PathBuf,
    /// the filepath to output the merged text to
    #[arg(short = 'o', long, alias = "out", verbatim_doc_comment)]
    output: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    initialize_logging(cli.log_level);

    let result = match &cli.command {
        Commands::Merge(m) => merge(&m.text_a, &m.text_b, &m.output),
        #[cfg(debug_assertions)]
        Commands::Debug => debug(),
    };

    match result {
        Ok(_) => println!("done!"),
        Err(e) => {
            println!("command execution failed:\nerror: {0}\nsource: {1:#?}\nroot cause: {2}\nbacktrace: {3}", e, e.source(), e.root_cause(), e.backtrace());
            exit(1);
        }
    }
}

fn merge(text_a: &Path, text_b: &Path, output: &Path) -> Result<()> {
    unimplemented!();
    Ok(())
}

fn debug() -> Result<()> {
    Ok(())
}
