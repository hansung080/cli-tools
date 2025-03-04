use std::fs;
use std::io::ErrorKind;
use std::sync::Once;
use assert_cmd::Command;
use anyhow::{Error, Result};

pub fn assert_command(cmd: &str, args: &[&str], expected_file: &str, init: &Once) -> Result<()> {
    let expected_file = format!("target/tests/expected/echo/{expected_file}");
    let expected = match fs::read_to_string(&expected_file) {
        Ok(content) => content,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                init.call_once(|| {
                    Command::new(format!("tests/expected/{cmd}.sh")).assert().success();
                });
                fs::read_to_string(expected_file)?
            },
            _ => return Err(Error::new(e)),
        }
    };
    let output = Command::cargo_bin(cmd)?
        .args(args)
        .output()?;
    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout)?, expected);
    Ok(())
}