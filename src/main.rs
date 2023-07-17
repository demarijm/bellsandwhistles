
use clap::Parser;
#[derive(Parser)]
#[command(name = "clap-test")]
#[command(author = "Demari Miller <demarijmiller@gmail.com>")]
#[command(version = "0.01")]
#[command(about = "Does awesome things", long_about = None)]

struct Cli {
    #[arg(long)]
    two: String,
    #[arg(long)]
    one: String,
}

fn main() {
    let cli = Cli::parse();


    println!("one: {:?}", cli.one);
    println!("two: {:?}", cli.two);
}
