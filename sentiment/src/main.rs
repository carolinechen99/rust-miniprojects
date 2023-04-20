use sentiment::analyze;

use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Caroline Chen",
    about = "CLI for Sentiment Analysis"
)]

struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(about = "Sentence to analyze")]
    Analyze {
        #[clap(short, long)]
        sentence: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Analyze { sentence }) => {
            let result = analyze(sentence);
            println!("Sentiment score: {}", result.score);
            println!("comparative score: {}", result.comparative);
            println!("Positive words: {:?}", result.positive.words);
            println!("Negative words: {:?}", result.negative.words);
        }
        None => {
            println!("No command given");
        }
    }
}

