use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    LongestParlindromicSubstring {
        #[clap(short, long)]
        input: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::LongestParlindromicSubstring { input }) => {
            let res = longest_parlidrome::longest_palindromic_substring(&input);
            println!("{res}");
        }
        None => {
            println!("No command given");
        }
    }
}