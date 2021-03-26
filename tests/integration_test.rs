use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn run_with_defaults() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("somethingsay")
        .expect("binary exists")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, I'm Mainforce!!"));
    Ok(())
}

#[test]
fn fail_on_non_existing_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("somethingsay")
        .expect("binary exists")
        .args(&["-f", "I/like/green-color.txt"])
        .assert()
        .failure();
    Ok(())
}