use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Caroline Chen",
    about = "CLI for Dictionary"
)]

struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(about = "Word to look up")]
    Lookup {
        #[clap(short, long)]
        word: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Lookup { word }) => {
            let result = dict::get_definition(&word);
            println!("{result}");
        }
        None => {
            println!("No command given");
        }
    }
}
