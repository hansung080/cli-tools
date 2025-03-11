mod utils;

use std::sync::Once;
use assert_cmd::Command;
use anyhow::Result;
use predicates::prelude::*;

const CMD: &str = "echo";

fn assert_echo(args: &[&str], expected_file: &str) -> Result<()> {
    static INIT: Once = Once::new();
    utils::assert_command(CMD, args, None, expected_file, &INIT)
}

#[test]
fn echo_help() -> Result<()> {
    for arg in &["-h", "--help"] {
        Command::cargo_bin(CMD)?
            .arg(arg)
            .assert()
            .success()
            .stdout(predicate::str::contains("Usage:"));
    }
    Ok(())
}

#[test]
fn echo_version() -> Result<()> {
    for arg in &["-V", "--version"] {
        Command::cargo_bin(CMD)?
            .arg(arg)
            .assert()
            .success()
            .stdout(predicate::str::is_match(format!("{CMD} [0-9]+[.][0-9]+[.][0-9]+.*"))?);
    }
    Ok(())
}

#[test]
fn echo_unknown_option() -> Result<()> {
    Command::cargo_bin(CMD)?
        .arg("-z")
        .assert()
        .failure()
        .stderr(predicate::str::contains("error:"));
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
fn echo_no_args_n() -> Result<()> {
    Command::cargo_bin(CMD)?
        .arg("-n")
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
    Ok(())
}

#[test]
fn echo_fruit_1() -> Result<()> {
    assert_echo(&["apple    banana"], "fruit_1.out")
}

#[test]
fn echo_fruit_2() -> Result<()> {
    assert_echo(&["apple", "banana"], "fruit_2.out")
}

#[test]
fn echo_fruit_1_n() -> Result<()> {
    assert_echo(&["-n", "apple    banana"], "fruit_1.n.out")
}

#[test]
fn echo_fruit_2_n() -> Result<()> {
    assert_echo(&["apple", "banana", "-n"], "fruit_2.n.out")
}