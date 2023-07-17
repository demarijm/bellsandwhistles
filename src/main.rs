
use clap::Parser;
#[derive(Parser)]
#[command(name = "MyApp")]
#[command(author = "Kevin K. <kbknapp@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Does awesome things", long_about = None)]

struct Cli {
    #[arg(long)]
    two: String,
    #[arg(long)]
    one: String,
}

fn main() {
    let cli = Cli::parse()


    println!("one: {:?}", cli.one);
}
