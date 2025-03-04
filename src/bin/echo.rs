use std::env;
use clap::{Arg, ArgAction, Command, Parser};

fn main() {
    run(Args::select());
}

#[derive(Parser, Debug)]
#[command(name = "echo", version = "0.1.0 (clap-derive)", author, about)]
/// Rust version of `echo` (clap-derive)
struct Args {
    /// Arguments to print to the standard output
    #[arg(required = false)]
    args: Vec<String>,

    /// Do not print the trailing newline character
    #[arg(short = 'n')]
    omit_newline: bool,
}

impl Args {
    fn select() -> Self {
        if env::var("CLAP_BUILDER").is_ok() {
            Self::build()
        } else {
            Self::parse()
        }
    }

    fn build() -> Self {
        let matches = Command::new("echo")
            .version("0.1.0 (clap-builder)")
            .author("Han-Seong Kwon <hansung080@hanmail.net>")
            .about("Rust version of `echo` (clap-builder)")
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
        Self { args, omit_newline }
    }
}

fn run(args: Args) {
    print!("{}{}", args.args.join(" "), if args.omit_newline { "" } else { "\n" });
}