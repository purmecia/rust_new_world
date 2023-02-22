/*Command-line interface for Basic Calculator */
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    IntegerToRoman {
        #[clap(short, long)]
        input: i32,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::IntegerToRoman { input }) => {
            let res = InttoRoman::int_to_roman(input);
            println!("{res}");
        }
        None => {
            println!("No command given");
        }
    }
}