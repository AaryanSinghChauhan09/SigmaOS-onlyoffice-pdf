#![forbid(unsafe_code)]
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "onlyoffice-pdf", about = "SigmaOS OnlyOffice PDF editor")]
struct Cli { #[command(subcommand)] cmd: Commands }
#[derive(Subcommand)]
enum Commands { List, Status }
fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::List   => println!("onlyoffice-pdf: no entries configured."),
        Commands::Status => println!("onlyoffice-pdf: ready."),
    }
}
