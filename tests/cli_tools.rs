use std::process;
use assert_cmd::Command;
use pretty_assertions::assert_eq;

#[test]
fn hello1() {
    let mut cmd = process::Command::new("target/debug/hello");
    let output = cmd.output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout, "Hello, world!\n");
}

#[test]
fn hello2() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    let output = cmd.output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout, "Hello, world!\n");
}

#[test]
fn r#true() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn r#false() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}