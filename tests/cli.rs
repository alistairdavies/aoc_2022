use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;


#[test]
fn runs_day_0() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("aoc_2022")?;

    cmd.arg("0");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Day 0!"));

    Ok(())
}


#[test]
fn invalid_day_number() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("aoc_2022")?;

    cmd.arg("9999");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not implemented yet"));
    Ok(())
}
