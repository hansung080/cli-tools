mod utils;

use std::fs;
use std::sync::Once;
use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use rand::distr::Alphanumeric;
use rand::Rng;

const CMD: &str = "cat";

const FILE_EMPTY: &str = "tests/input/cat/empty.txt";
const FILE_FOX: &str = "tests/input/cat/fox.txt";
const FILE_SPIDERS: &str = "tests/input/cat/spiders.txt";
const FILE_BUSTLE: &str = "tests/input/cat/the_bustle.txt";
const FILE_NO_PERMISSION: &str = "target/tests/input/cat/no_permission.txt";

fn assert_cat(args: &[&str], expected_file: &str) -> Result<()> {
    assert_cat_stdin(args, "", expected_file)
}

fn assert_cat_stdin(args: &[&str], stdin_file: &str, expected_file: &str) -> Result<()> {
    static INIT: Once = Once::new();
    let stdin_file = if stdin_file.is_empty() { None } else { Some(stdin_file) };
    utils::assert_command(CMD, args, stdin_file, expected_file, &INIT)
}

#[test]
fn cat_help() -> Result<()> {
    for opt in ["-h", "--help"] {
        Command::cargo_bin(CMD)?
            .arg(opt)
            .assert()
            .success()
            .stdout(predicate::str::contains("Usage:"));
    }
    Ok(())
}

#[test]
fn cat_conflicted_options_nb() -> Result<()> {
    Command::cargo_bin(CMD)?
        .args(["-n", "-b"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("error:"));
    Ok(())
}

#[test]
fn cat_absent_file() -> Result<()> {
    let filename = gen_absent_filename();
    Command::cargo_bin(CMD)?
        .arg(&filename)
        .assert()
        .failure()
        .stderr(predicate::str::is_match(format!("{CMD}: {filename}: .* [(]os error 2[)]"))?);
    Ok(())
}

fn gen_absent_filename() -> String {
    loop {
        let filename: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

#[test]
fn cat_no_permission() -> Result<()> {
    utils::create_file_if_not_exists(FILE_NO_PERMISSION, 0o000, "");
    Command::cargo_bin(CMD)?
        .arg(FILE_NO_PERMISSION)
        .assert()
        .failure()
        .stderr(predicate::str::is_match(format!("{CMD}: {FILE_NO_PERMISSION}: .* [(]os error 13[)]"))?);
    Ok(())
}

#[test]
fn cat_empty() -> Result<()> {
    assert_cat(&[FILE_EMPTY], "empty.out")
}

#[test]
fn cat_empty_n() -> Result<()> {
    assert_cat(&["-n", FILE_EMPTY], "empty.n.out")
}

#[test]
fn cat_empty_b() -> Result<()> {
    assert_cat(&["-b", FILE_EMPTY], "empty.b.out")
}

#[test]
fn cat_fox() -> Result<()> {
    assert_cat(&[FILE_FOX], "fox.out")
}

#[test]
fn cat_fox_n() -> Result<()> {
    assert_cat(&["--number", FILE_FOX], "fox.n.out")
}

#[test]
fn cat_fox_b() -> Result<()> {
    assert_cat(&["--number-nonblank", FILE_FOX], "fox.b.out")
}

#[test]
fn cat_spiders() -> Result<()> {
    assert_cat(&[FILE_SPIDERS], "spiders.out")
}

#[test]
fn cat_spiders_n() -> Result<()> {
    assert_cat(&[FILE_SPIDERS, "-n"], "spiders.n.out")
}

#[test]
fn cat_spiders_b() -> Result<()> {
    assert_cat(&[FILE_SPIDERS, "-b"], "spiders.b.out")
}

#[test]
fn cat_bustle() -> Result<()> {
    assert_cat(&[FILE_BUSTLE], "the_bustle.out")
}

#[test]
fn cat_bustle_n() -> Result<()> {
    assert_cat(&[FILE_BUSTLE, "--number"], "the_bustle.n.out")
}

#[test]
fn cat_bustle_b() -> Result<()> {
    assert_cat(&[FILE_BUSTLE, "--number-nonblank"], "the_bustle.b.out")
}

#[test]
fn cat_all() -> Result<()> {
    assert_cat(&[FILE_EMPTY, FILE_FOX, FILE_SPIDERS, FILE_BUSTLE], "all.out")
}

#[test]
fn cat_all_n() -> Result<()> {
    assert_cat(&["-n", FILE_EMPTY, FILE_FOX, FILE_SPIDERS, FILE_BUSTLE], "all.n.out")
}

#[test]
fn cat_all_b() -> Result<()> {
    assert_cat(&["-b", FILE_EMPTY, FILE_FOX, FILE_SPIDERS, FILE_BUSTLE], "all.b.out")
}

#[test]
fn cat_bustle_stdin() -> Result<()> {
    assert_cat_stdin(&["-"], FILE_BUSTLE, "the_bustle.stdin.out")
}

#[test]
fn cat_bustle_stdin_n() -> Result<()> {
    assert_cat_stdin(&["-n", "-"], FILE_BUSTLE, "the_bustle.stdin.n.out")
}

#[test]
fn cat_bustle_stdin_b() -> Result<()> {
    assert_cat_stdin(&["-b", "-"], FILE_BUSTLE, "the_bustle.stdin.b.out")
}