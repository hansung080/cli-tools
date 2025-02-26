use assert_cmd::Command;

#[test]
fn r#true() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}