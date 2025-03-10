use std::io::BufRead;
use clap::{Arg, ArgAction, Command, Parser};
use anyhow::Result;
use cli_tools::args::{Build, Select};
use cli_tools::process::{ExitCode, HandleAndExit};
use cli_tools::utils;

const TAG: &str = "cat";

fn main() {
    run(Args::select()).handle_and_exit(TAG);
}

#[derive(Parser, Debug)]
#[command(name = "cat", version = "0.1.0 (clap-derive)", author, about)]
/// Rust version of `cat` (clap-derive)
struct Args {
    /// Files to read and print to the standard output, a dash (-) or absence represents the standard input
    #[arg(value_name = "FILES", default_value = "-")]
    filenames: Vec<String>,

    /// Number the output lines, starting at 1
    #[arg(short = 'n', long = "number", conflicts_with = "number_nonblank")]
    number: bool,

    /// Number the non-blank output lines, starting at 1
    #[arg(short = 'b', long = "number-nonblank")]
    number_nonblank: bool,
}

impl Build for Args {
    fn build() -> Self {
        let matches = Command::new("cat")
            .version("0.1.0 (clap-builder)")
            .author("Han-Seong Kwon <hansung080@hanmail.net>")
            .about("Rust version of `cat` (clap-builder)")
            .arg(
                Arg::new("files")
                    .value_name("FILES")
                    .help("Files to read and print to the standard output, a dash (-) or absence represents the standard input")
                    .default_value("-")
                    .num_args(1..),
            )
            .arg(
                Arg::new("number")
                    .short('n')
                    .long("number")
                    .help("Number the output lines, starting at 1")
                    .action(ArgAction::SetTrue)
                    .conflicts_with("number_nonblank"),
            )
            .arg(
                Arg::new("number_nonblank")
                    .short('b')
                    .long("number-nonblank")
                    .help("Number the non-blank output lines, starting at 1")
                    .action(ArgAction::SetTrue),
            )
            .get_matches();
        Self {
            filenames: matches.get_many("files").unwrap().cloned().collect(),
            number: matches.get_flag("number"),
            number_nonblank: matches.get_flag("number_nonblank"),
        }
    }
}

impl Select for Args {}

fn run(args: Args) -> Result<ExitCode> {
    let mut code = ExitCode::Ok;
    for filename in args.filenames {
        match utils::open(&filename) {
            Ok(file) => {
                let mut num_nb = 0;
                for (num, line) in file.lines().enumerate() {
                    let line = line?;
                    if args.number {
                        println!("{:6}\t{line}", num + 1);
                    } else if args.number_nonblank {
                        if line.is_empty() {
                            println!();
                        } else {
                            num_nb += 1;
                            println!("{num_nb:6}\t{line}");
                        }
                    } else {
                        println!("{line}");
                    }
                }
            },
            Err(e) => {
                eprintln!("{TAG}: {filename}: {e}");
                code = ExitCode::BasicError;
            },
        }
    }
    Ok(code)
}
