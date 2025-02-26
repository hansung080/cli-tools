use assert_cmd::Command;

#[test]
fn r#false() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}