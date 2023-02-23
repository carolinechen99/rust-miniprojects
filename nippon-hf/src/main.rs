use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Caroline Chen",
    about = "Translate between English and Japanese."
)]

struct Cli {
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Parser)]
enum Command {
    #[clap(
        version = "1.0",
        author = "Caroline Chen",
        about = "Translate from English to Japanese."
    )]
    EnToNihongo {
        #[clap(short, long)]
        text: String,
    },
    #[clap(
        version = "1.0",
        author = "Caroline Chen",
        about = "Translate from Japanese to English."
    )]
    NihongoToEn {
        #[clap(short, long)]
        text: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Command::EnToNihongo { text }) => {
            println!("Translating from English to Japanese...");
            println!("Input: {text}");
            println!("Output: {:?}", nippon_hf::translate_to_japanese(&text).unwrap());
        }
        Some(Command::NihongoToEn { text }) => {
            println!("Translating from Japanese to English...");
            println!("Input: {text}");
            println!("Output: {:?}", nippon_hf::translate_to_japanese(&text).unwrap());
        }
        None => println!("No command was used."),
    }
}
