use clap::{Arg, ArgAction, Command, Parser};
use cli_tools::args::{Build, Select};

fn main() {
    run(Args::select());
}

#[derive(Parser, Debug)]
#[command(name = "echo", version = "0.1.0 (clap-derive)", author, about)]
/// Rust version of `echo` (clap-derive)
struct Args {
    /// Arguments to print to the standard output
    #[arg(value_name = "ARGS", required = false)]
    args: Vec<String>,

    /// Do not print the trailing newline character
    #[arg(short = 'n')]
    omit_newline: bool,
}

impl Build for Args {
    fn build() -> Self {
        let matches = Command::new("echo")
            .version("0.1.0 (clap-builder)")
            .author("Han-Seong Kwon <hansung080@hanmail.net>")
            .about("Rust version of `echo` (clap-builder)")
            .arg(
                Arg::new("args")
                    .value_name("ARGS")
                    .required(false)
                    .num_args(0..)
                    .help("Arguments to print to the standard output"),
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

impl Select for Args {}

fn run(args: Args) {
    print!("{}{}", args.args.join(" "), if args.omit_newline { "" } else { "\n" });
}