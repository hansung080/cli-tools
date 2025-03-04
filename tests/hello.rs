use std::process;
use assert_cmd::Command;
use pretty_assertions::assert_eq;

const CMD: &str = "hello";

#[test]
fn hello1() {
    let mut cmd = process::Command::new(format!("target/debug/{CMD}"));
    let output = cmd.output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout, "Hello, world!\n");
}

#[test]
fn hello2() {
    let mut cmd = Command::cargo_bin(CMD).unwrap();
    let output = cmd.output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout, "Hello, world!\n");
}