use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(
        short,
        long,
        help = "Set alphabets in colon-separated list",
        value_delimiter = ':'
    )]
    alphabets: Option<Vec<String>>,

    #[arg(short, long, default_value_t = 0, help = "Set shift for alphabets")]
    shiftalpha: usize,

    #[arg(short = 'e', long, default_value_t = 0, help = "Set ending alphabet")]
    alphaend: usize,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Convert number to word")]
    Encode {
        num: u64,

        #[arg(short = 'l', long, default_value_t = 1, help = "Set minimum length")]
        minlength: usize,
    },

    #[command(about = "Convert word to number")]
    Decode { word: String },

    #[command(about = "Check if word is banana")]
    Check {
        word: String,

        #[arg(short, long, default_value_t = false)]
        quiet: bool,
    },

    #[command(about = "Generate random banana")]
    Random {
        #[arg(short = 'l', long, default_value_t = 1, help = "Set minimum length")]
        minlength: usize,
    },
}

fn main() {
    let args = Cli::parse();

    let result = match args.command {
        Commands::Encode { num, minlength } => banana::encode(
            num,
            &banana::EncodeParams {
                alphabet_shift: Some(args.shiftalpha),
                alphabet_end: Some(args.alphaend),
                min_length: Some(minlength),
                alphabets: args.alphabets,
            },
        ),
        Commands::Decode { word } => banana::decode(
            &word,
            &banana::DecodeParams {
                alphabet_shift: Some(args.shiftalpha),
                alphabet_end: Some(args.alphaend),
                alphabets: args.alphabets,
            },
        )
        .map(|a| a.to_string()),
        Commands::Check { word, quiet } => match banana::is_valid(
            &word,
            &banana::DecodeParams {
                alphabet_shift: Some(args.shiftalpha),
                alphabet_end: Some(args.alphaend),
                alphabets: args.alphabets,
            },
        ) {
            true => {
                if !quiet {
                    println!("yes");
                }
                std::process::exit(0);
            }
            false => {
                if !quiet {
                    println!("no");
                }
                std::process::exit(1);
            }
        },
        Commands::Random { minlength } => banana::random(&banana::EncodeParams {
            alphabet_shift: Some(args.shiftalpha),
            alphabet_end: Some(args.alphaend),
            min_length: Some(minlength),
            alphabets: args.alphabets,
        }),
    };

    match result {
        Ok(v) => println!("{}", v),
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}
