use std::io::{BufRead, Read};
use clap::Parser;
use anyhow::Result;
use cli_tools::args::{Build, Select};
use cli_tools::process::{ExitCode, HandleAndExit};
use cli_tools::utils;

const TAG: &str = "head";

fn main() {
    run(Args::select()).handle_and_exit(TAG);
}

#[derive(Parser, Debug)]
#[command(name = "head", version = "0.1.0 (clap-derive)", author, about)]
/// Rust version of `head` (clap-derive)
struct Args {
    /// Files to read and print to the standard output, a dash (-) or absence represents the standard input
    #[arg(value_name = "FILES", default_value = "-")]
    filenames: Vec<String>,

    /// Print lines of each of the specified files
    #[arg(
        short = 'n',
        long = "lines",
        value_name = "LINES",
        default_value = "10",
        value_parser = clap::value_parser!(u64).range(1..),
    )]
    lines: u64,

    /// Print bytes of each of the specified files
    #[arg(
        short = 'c',
        long = "bytes",
        value_name = "BYTES",
        value_parser = clap::value_parser!(u64).range(1..),
        conflicts_with = "lines",
    )]
    bytes: Option<u64>,
}

impl Build for Args {
    fn build() -> Self {
        todo!()
    }
}

impl Select for Args {}

fn run(args: Args) -> Result<ExitCode> {
    let mut code = ExitCode::Ok;
    let n_filenames = args.filenames.len();
    for (i, filename) in args.filenames.iter().enumerate() {
        match utils::open(filename) {
            Ok(mut file) =>  {
                if n_filenames > 1 {
                    println!("{}==> {filename} <==", if i > 0 { "\n" } else { "" });
                }
                if let Some(bytes) = args.bytes {
                    let mut buf = vec![0; bytes as usize];
                    let n = file.read(&mut buf)?;
                    print!("{}", String::from_utf8_lossy(&buf[..n]));
                } else {
                    let mut line = String::new();
                    for _ in 0..args.lines {
                        if file.read_line(&mut line)? == 0 {
                            break;
                        }
                        print!("{line}");
                        line.clear();
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