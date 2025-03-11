#![allow(unused)]

pub mod consts;

use std::borrow::Cow;
use std::fs::{self, File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::io::{self, ErrorKind, Read, Write};
use std::path::Path;
use std::sync::Once;
use assert_cmd::Command;
use anyhow::{Error, Result};
use rand::distr::Alphanumeric;
use rand::Rng;

pub fn assert_command(
    cmd: &str,
    args: &[&str],
    stdin_file: Option<&str>,
    expected_file: &str,
    init: &Once
) -> Result<()> {
    let expected_file = format!("target/tests/expected/{cmd}/{expected_file}");
    let expected = match read_to_string_lossy(&expected_file) {
        Ok(content) => content,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                init.call_once(|| {
                    Command::new(format!("tests/expected/{cmd}.sh")).assert().success();
                });
                read_to_string_lossy(expected_file)?
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
    assert_eq!(String::from_utf8_lossy(&output.stdout), expected);
    Ok(())
}

pub fn read_to_string_lossy<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut buf = Vec::new();
    File::open(path)?.read_to_end(&mut buf)?;
    Ok(String::from_utf8_lossy(&buf).to_string())
}

pub fn gen_random_string() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

pub fn gen_absent_filename() -> String {
    loop {
        let filename: String = gen_random_string();
        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

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