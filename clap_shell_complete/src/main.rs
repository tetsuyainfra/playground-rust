use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate};


#[derive(Parser)]
struct Cli {

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    ///
    GenerateComplete {
        #[arg(help = "The shell to generate the completion script for")]
        shell: clap_complete::Shell,
    },

    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}


pub fn build_cli() -> clap::Command {
    Cli::command()
}


fn main() {
    let cli = Cli::parse();


    match &cli.command {
        Some(Commands::GenerateComplete { shell }) => {
            generate(*shell, &mut build_cli(), "myapp", &mut std::io::stdout());
        }
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing values...");
            } else {
                println!("Not printing testing values...");
            }
        }
        None => {}
    }


}