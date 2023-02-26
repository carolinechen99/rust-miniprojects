use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Caroline Chen",
    about = "Tool to get IP address"
)]

struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(about = "Get the local IP address of system")]
    Localip {
        // #[clap(short, long)]
    },
    #[clap(
        about = "Retrieve all the available network interfaces from both, the AF_INET and the AF_INET6 family"
    )]
    Listnetifas {
        // #[clap(short, long)]
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Localip { .. }) => {
            get_ip::get_local_ip()
        }
        Some(Commands::Listnetifas { .. }) => {
            get_ip::list_netifas();
        }
        None => {
            println!("No command provided");
        }
    }
}
