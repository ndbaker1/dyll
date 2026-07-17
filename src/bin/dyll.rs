use clap::Parser;
use dyll::{
    cli::{Cli, Commands},
    providers,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Header(ref args) => providers::header::run(&cli, args),
    }
}
