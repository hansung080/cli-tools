use assert_cmd::Command;

const CMD: &str = "true";

#[test]
fn r#true() {
    let mut cmd = Command::cargo_bin(CMD).unwrap();
    cmd.assert().success();
}