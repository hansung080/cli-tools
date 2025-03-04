mod utils;

use std::sync::Once;
use assert_cmd::Command;
use anyhow::Result;
use utils::assert_command;

const CMD: &str = "echo";

fn assert_echo(args: &[&str], expected_file: &str) -> Result<()> {
    static INIT: Once = Once::new();
    assert_command(CMD, args, expected_file, &INIT)
}

#[test]
fn echo_invalid_option() -> Result<()> {
    Command::cargo_bin(CMD)?
        .arg("-z")
        .assert()
        .failure()
        .stderr(predicates::str::contains("Usage:"));
    Ok(())
}

#[test]
fn echo_no_args() -> Result<()> {
    Command::cargo_bin(CMD)?
        .assert()
        .success()
        .stdout("\n");
    Ok(())
}

#[test]
fn echo_fruit_1() -> Result<()> {
    assert_echo(&["apple    banana"], "fruit_1.txt")
}

#[test]
fn echo_fruit_2() -> Result<()> {
    assert_echo(&["apple", "banana"], "fruit_2.txt")
}

#[test]
fn echo_n_no_args() -> Result<()> {
    Command::cargo_bin(CMD)?
        .arg("-n")
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
    Ok(())
}

#[test]
fn echo_n_fruit_1() -> Result<()> {
    assert_echo(&["-n", "apple    banana"], "fruit_1.n.txt")
}

#[test]
fn echo_n_fruit_2() -> Result<()> {
    assert_echo(&["apple", "banana", "-n"], "fruit_2.n.txt")
}