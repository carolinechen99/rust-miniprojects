use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short, long)]
    x: f32,
    #[structopt(short, long)]
    y: f32,
    #[structopt(subcommand)]
    operator: Operator,
}

#[derive(Debug, StructOpt)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

fn main() {
    let args = Cli::from_args();
    let x = args.x;
    let y = args.y;
    let operator = args.operator;

    let result = match operator {
        Operator::Add => x + y,
        Operator::Sub => x - y,
        Operator::Mul => x * y,
        Operator::Div => x / y,
    };

    println!("{result}");
}
