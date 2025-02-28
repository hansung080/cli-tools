use std::fs;
use std::io::ErrorKind;
use assert_cmd::Command;
use anyhow::{Error, Result};
use pretty_assertions::assert_eq;

#[test]
fn echo_invalid_option() -> Result<()> {
    Command::cargo_bin("echo")?
        .arg("-z")
        .assert()
        .failure()
        .stderr(predicates::str::contains("Usage:"));
    Ok(())
}

#[test]
fn echo_no_args() -> Result<()> {
    Command::cargo_bin("echo")?
        .assert()
        .success()
        .stdout("\n");
    Ok(())
}

#[test]
fn echo_1_arg() -> Result<()> {
    assert_eq_file(&["apple    banana"], "target/tests/expected/echo_1.txt")
}

#[test]
fn echo_2_args() -> Result<()> {
    assert_eq_file(&["apple", "banana"], "target/tests/expected/echo_2.txt")
}

#[test]
fn echo_n_no_args() -> Result<()> {
    Command::cargo_bin("echo")?
        .arg("-n")
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
    Ok(())
}

#[test]
fn echo_n_1_arg() -> Result<()> {
    assert_eq_file(&["-n", "apple    banana"], "target/tests/expected/echo_n_1.txt")
}

#[test]
fn echo_n_2_args() -> Result<()> {
    assert_eq_file(&["apple", "banana", "-n"], "target/tests/expected/echo_n_2.txt")
}

fn assert_eq_file(args: &[&str], expected_path: &str) -> Result<()> {
    let expected = match fs::read_to_string(expected_path) {
        Ok(content) => content,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                Command::new("tests/scripts/echo.sh").assert().success();
                fs::read_to_string(expected_path)?
            },
            _ => return Err(Error::new(e)),
        }
    };
    let output = Command::cargo_bin("echo")?
        .args(args)
        .output()?;
    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout)?, expected);
    Ok(())
}

