use anyhow::{Context, Error, Result};
use aoc_2022::*;
use clap::Parser;

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
        0 => day_0::Day0 {}
            .run()
            .with_context(|| "Failed to run day 0.")?,
        1 => {
            let result = day_1::Day1::default()
                .run()
                .with_context(|| "Failed to run day 1.")?;
            println!("Max calories: {}", result);
        }
        2 => {
            day_2::Day2::default()
                .run()
                .with_context(|| "Failed to run day 2.")?;
        }
        3 => {
            day_3::Day3::default()
                .run()
                .with_context(|| "Failed to run day 3.")?;
        }
        4 => {
            day_4::Day4::default()
                .run()
                .with_context(|| "Failed to run day 4.")?;
        }
        _ => {
            return Err(Error::msg(format!(
                "Day {} is not implemented yet",
                day_number
            )))
        }
    }
    Ok(())
}
