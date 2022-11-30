use aoc_2022::day_0;
use clap::Parser;
use anyhow::{Result, Context, Error};

#[derive(Parser)]
struct Cli {
    day: usize,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.day).with_context(|| "Failed to run.")?;
    Ok(())
}

fn run(day_number: usize) -> Result<()> {
    match day_number {
        0 => day_0::run(),
        _ => return Err(Error::msg(format!("Day {} is not implemented yet", day_number))),
    }
    Ok(())
}
