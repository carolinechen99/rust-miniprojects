//import necessary libraries
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Caroline Chen",
    about = "A Hugging Face Translation Tool in Rust"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(
        version = "1.0",
        author = "Caroline Chen",
        about = "Translate a file from Japanese to English"
    )]
    Translate {
        #[clap(short, long)]
        path: String,
    },
    Print {
        #[clap(short, long)]
        path: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Translate { path }) => {
            println!("Translating file from Japanese to English...");
            println!("Path: {}", path);
        }
        Some(Commands::Print { path }) => {
            println!("Printing file...");
            println!("Path: {}", path);
        }
        None => {
            println!("No command specified. Please specify a command.");
        }
    }
}
