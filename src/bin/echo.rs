use clap::{Arg, ArgAction, Command, Parser};

enum ClapPattern {
    #[allow(unused)] Builder,
    #[allow(unused)] Derive,
}

// Select the clap pattern.
const CLAP_PATTERN: ClapPattern = ClapPattern::Derive;

fn main() {
    match CLAP_PATTERN {
        ClapPattern::Builder => run_with_clap_builder(),
        ClapPattern::Derive => run_with_clap_derive(),
    };
}

fn run_with_clap_builder() {
    let matches = Command::new("echo")
        .version("0.1.0")
        .author("Han-Seong Kwon <hansung080@hanmail.net>")
        .about("Rust version of `echo`")
        .arg(
            Arg::new("args")
                .value_name("ARGS")
                .help("Arguments to print to the standard output")
                .required(false)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Do not print the trailing newline character"),
        )
        .get_matches();

    let args: Vec<String> = match matches.get_many("args") {
        Some(args) => args.cloned().collect(),
        None => vec![],
    };
    let omit_newline = matches.get_flag("omit_newline");

    print!("{}{}", args.join(" "), if omit_newline { "" } else { "\n" });
}

#[derive(Debug, Parser)]
#[command(name = "echo", version, author, about)]
/// Rust version of `echo`
struct Args {
    /// Arguments to print to the standard output
    #[arg(required = false)]
    args: Vec<String>,

    /// Do not print the trailing newline character
    #[arg(short = 'n')]
    omit_newline: bool,
}

fn run_with_clap_derive() {
    let args = Args::parse();
    print!("{}{}", args.args.join(" "), if args.omit_newline { "" } else { "\n" });
}