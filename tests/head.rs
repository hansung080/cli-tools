mod utils;

use std::sync::Once;
use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use utils::consts::FILE_NO_PERMISSION;

const CMD: &str = "head";

const FILE_EMPTY: &str = "tests/input/head/empty.txt";
const FILE_ONE: &str = "tests/input/head/one.txt";
const FILE_TWO: &str = "tests/input/head/two.txt";
const FILE_THREE: &str = "tests/input/head/three.txt";
const FILE_TWELVE: &str = "tests/input/head/twelve.txt";

fn assert_head(args: &[&str], expected_file: &str) -> Result<()> {
    assert_head_stdin(args, "", expected_file)
}

fn assert_head_stdin(args: &[&str], stdin_file: &str, expected_file: &str) -> Result<()> {
    static INIT: Once = Once::new();
    let stdin_file = if stdin_file.is_empty() { None } else { Some(stdin_file) };
    utils::assert_command(CMD, args, stdin_file, expected_file, &INIT)
}

#[test]
fn head_invalid_options() -> Result<()> {
    let args_list: &[&[&str]] = &[
        &["-n"],
        &["-n", "a", FILE_EMPTY],
        &["-n", "0", FILE_EMPTY],
        &["-c"],
        &["-c", "a", FILE_EMPTY],
        &["-c", "0", FILE_EMPTY],
        &["-n", "1", "-c", "1", FILE_EMPTY],
    ];

    for args in args_list {
        Command::cargo_bin(CMD)?
            .args(*args)
            .assert()
            .failure()
            .stderr(predicate::str::contains("error:"));
    }
    Ok(())
}

#[test]
fn head_absent_file() -> Result<()> {
    let filename = utils::gen_absent_filename();
    Command::cargo_bin(CMD)?
        .args(&[FILE_EMPTY, &filename, FILE_ONE])
        .assert()
        .failure()
        .stderr(predicate::str::is_match(format!("{CMD}: {filename}: .* [(]os error 2[)]"))?);
    Ok(())
}

#[test]
fn head_no_permission() -> Result<()> {
    utils::create_file_if_not_exists(FILE_NO_PERMISSION, 0o000, "");
    Command::cargo_bin(CMD)?
        .args(&[FILE_EMPTY, FILE_NO_PERMISSION, FILE_ONE])
        .assert()
        .failure()
        .stderr(predicate::str::is_match(format!("{CMD}: {FILE_NO_PERMISSION}: .* [(]os error 13[)]"))?);
    Ok(())
}

#[test]
fn head_empty() -> Result<()> {
    assert_head(&[FILE_EMPTY], "empty.out")
}

#[test]
fn head_empty_n2() -> Result<()> {
    assert_head(&["-n", "2", FILE_EMPTY], "empty.n2.out")
}

#[test]
fn head_empty_n4() -> Result<()> {
    assert_head(&["-n", "4", FILE_EMPTY], "empty.n4.out")
}

#[test]
fn head_empty_c1() -> Result<()> {
    assert_head(&["-c", "1", FILE_EMPTY], "empty.c1.out")
}

#[test]
fn head_empty_c2() -> Result<()> {
    assert_head(&["-c", "2", FILE_EMPTY], "empty.c2.out")
}

#[test]
fn head_empty_c4() -> Result<()> {
    assert_head(&["-c", "4", FILE_EMPTY], "empty.c4.out")
}

#[test]
fn head_one() -> Result<()> {
    assert_head(&[FILE_ONE], "one.out")
}

#[test]
fn head_one_n2() -> Result<()> {
    assert_head(&["-n", "2", FILE_ONE], "one.n2.out")
}

#[test]
fn head_one_n4() -> Result<()> {
    assert_head(&["-n", "4", FILE_ONE], "one.n4.out")
}

#[test]
fn head_one_c1() -> Result<()> {
    assert_head(&["-c", "1", FILE_ONE], "one.c1.out")
}

#[test]
fn head_one_c2() -> Result<()> {
    assert_head(&["-c", "2", FILE_ONE], "one.c2.out")
}

#[test]
fn head_one_c4() -> Result<()> {
    assert_head(&["-c", "4", FILE_ONE], "one.c4.out")
}

#[test]
fn head_two() -> Result<()> {
    assert_head(&[FILE_TWO], "two.out")
}

#[test]
fn head_two_n2() -> Result<()> {
    assert_head(&["--lines", "2", FILE_TWO], "two.n2.out")
}

#[test]
fn head_two_n4() -> Result<()> {
    assert_head(&["--lines", "4", FILE_TWO], "two.n4.out")
}

#[test]
fn head_two_c1() -> Result<()> {
    assert_head(&["--bytes", "1", FILE_TWO], "two.c1.out")
}

#[test]
fn head_two_c2() -> Result<()> {
    assert_head(&["--bytes", "2", FILE_TWO], "two.c2.out")
}

#[test]
fn head_two_c4() -> Result<()> {
    assert_head(&["--bytes", "4", FILE_TWO], "two.c4.out")
}

#[test]
fn head_three() -> Result<()> {
    assert_head(&[FILE_THREE], "three.out")
}

#[test]
fn head_three_n2() -> Result<()> {
    assert_head(&[FILE_THREE, "-n", "2"], "three.n2.out")
}

#[test]
fn head_three_n4() -> Result<()> {
    assert_head(&[FILE_THREE, "-n", "4"], "three.n4.out")
}

#[test]
fn head_three_c1() -> Result<()> {
    assert_head(&[FILE_THREE, "-c", "1"], "three.c1.out")
}

#[test]
fn head_three_c2() -> Result<()> {
    assert_head(&[FILE_THREE, "-c", "2"], "three.c2.out")
}

#[test]
fn head_three_c4() -> Result<()> {
    assert_head(&[FILE_THREE, "-c", "4"], "three.c4.out")
}

#[test]
fn head_twelve() -> Result<()> {
    assert_head(&[FILE_TWELVE], "twelve.out")
}

#[test]
fn head_twelve_n2() -> Result<()> {
    assert_head(&[FILE_TWELVE, "--lines", "2"], "twelve.n2.out")
}

#[test]
fn head_twelve_n4() -> Result<()> {
    assert_head(&[FILE_TWELVE, "--lines", "4"], "twelve.n4.out")
}

#[test]
fn head_twelve_c1() -> Result<()> {
    assert_head(&[FILE_TWELVE, "--bytes", "1"], "twelve.c1.out")
}

#[test]
fn head_twelve_c2() -> Result<()> {
    assert_head(&[FILE_TWELVE, "--bytes", "2"], "twelve.c2.out")
}

#[test]
fn head_twelve_c4() -> Result<()> {
    assert_head(&[FILE_TWELVE, "--bytes", "4"], "twelve.c4.out")
}

#[test]
fn head_all() -> Result<()> {
    assert_head(&[FILE_EMPTY, FILE_ONE, FILE_TWO, FILE_THREE, FILE_TWELVE], "all.out")
}

#[test]
fn head_all_n2() -> Result<()> {
    assert_head(&["-n", "2", FILE_EMPTY, FILE_ONE, FILE_TWO, FILE_THREE, FILE_TWELVE], "all.n2.out")
}

#[test]
fn head_all_n4() -> Result<()> {
    assert_head(&["-n", "4", FILE_EMPTY, FILE_ONE, FILE_TWO, FILE_THREE, FILE_TWELVE], "all.n4.out")
}

#[test]
fn head_all_c1() -> Result<()> {
    assert_head(&["-c", "1", FILE_EMPTY, FILE_ONE, FILE_TWO, FILE_THREE, FILE_TWELVE], "all.c1.out")
}

#[test]
fn head_all_c2() -> Result<()> {
    assert_head(&["-c", "2", FILE_EMPTY, FILE_ONE, FILE_TWO, FILE_THREE, FILE_TWELVE], "all.c2.out")
}

#[test]
fn head_all_c4() -> Result<()> {
    assert_head(&["-c", "4", FILE_EMPTY, FILE_ONE, FILE_TWO, FILE_THREE, FILE_TWELVE], "all.c4.out")
}

#[test]
fn head_one_stdin() -> Result<()> {
    assert_head_stdin(&[], FILE_ONE, "one.stdin.out")
}

#[test]
fn head_one_n2_stdin() -> Result<()> {
    assert_head_stdin(&["-n", "2", "-"], FILE_ONE, "one.stdin.n2.out")
}

#[test]
fn head_one_n4_stdin() -> Result<()> {
    assert_head_stdin(&["-n", "4"], FILE_ONE, "one.stdin.n4.out")
}

#[test]
fn head_one_c1_stdin() -> Result<()> {
    assert_head_stdin(&["-c", "1"], FILE_ONE, "one.stdin.c1.out")
}

#[test]
fn head_one_c2_stdin() -> Result<()> {
    assert_head_stdin(&["-c", "2", "-"], FILE_ONE, "one.stdin.c2.out")
}

#[test]
fn head_one_c4_stdin() -> Result<()> {
    assert_head_stdin(&["-c", "4"], FILE_ONE, "one.stdin.c4.out")
}