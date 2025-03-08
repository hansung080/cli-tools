use std::fs::{self, File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::io::{ErrorKind, Write};
use std::path::Path;
use std::sync::Once;
use assert_cmd::Command;
use anyhow::{Error, Result};

#[allow(unused)]
pub fn assert_command(
    cmd: &str,
    args: &[&str],
    stdin_file: Option<&str>,
    expected_file: &str,
    init: &Once
) -> Result<()> {
    let expected_file = format!("target/tests/expected/{cmd}/{expected_file}");
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

    let output = match stdin_file {
        Some(stdin_file) => {
            let stdin = fs::read_to_string(stdin_file)?;
            Command::cargo_bin(cmd)?
                .args(args)
                .write_stdin(stdin)
                .output()?
        },
        None => {
            Command::cargo_bin(cmd)?
                .args(args)
                .output()?
        },
    };

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout)?, expected);
    Ok(())
}

#[allow(unused)]
pub fn create_dir_if_not_exists<P: AsRef<Path>>(path: P, recursive: bool) {
    let path = path.as_ref();
    if !path.exists() {
        if recursive {
            fs::create_dir_all(path).expect(&format!("failed to create {}", path.display()));
        } else {
            fs::create_dir(path).expect(&format!("failed to create {}", path.display()));
        }
    }
}

#[allow(unused)]
pub fn create_file_if_not_exists<P: AsRef<Path>>(path: P, mode: u32, content: &str) -> Option<File> {
    let path = path.as_ref();
    if path.exists() {
        return None;
    }

    create_dir_if_not_exists(path.parent().unwrap(), true);

    let mut file = OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .append(true)
        .mode(mode)
        .open(path)
        .expect(&format!("failed to create {}", path.display()));

    if !content.is_empty() {
        file.write_all(content.as_bytes()).expect(&format!("failed to write {}", path.display()));
    }
    Some(file)
}