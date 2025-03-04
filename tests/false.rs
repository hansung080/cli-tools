use assert_cmd::Command;

const CMD: &str = "false";

#[test]
fn r#false() {
    let mut cmd = Command::cargo_bin(CMD).unwrap();
    cmd.assert().failure();
}