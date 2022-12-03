use anyhow::{Context, Error, Result};
use aoc_2022::*;
use clap::Parser;
use std::path::Path;

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
    let file_reader = Box::new(files::FileSystemReader {});
    match day_number {
        0 => day_0::Day0 {}
            .run()
            .with_context(|| "Failed to run day 0.")?,
        1 => {
            let input_file = Path::new("assets/day_1/input.txt");
            let result = day_1::Day1 {
                file_reader,
                file_path: input_file,
            }
            .run()
            .with_context(|| "Failed to run day 1.")?;
            println!("Max calories: {}", result);
        }
        2 => {
            let input_file = Path::new("assets/day_2/input.txt");
            day_2::Day2 {
                file_reader,
                file_path: input_file,
            }
            .run()
            .with_context(|| "Failed to run day 2.")?;
        }
        3 => {
            let input_file = Path::new("assets/day_3/input.txt");
            day_3::Day3 {
                file_reader,
                file_path: input_file,
            }
            .run()
            .with_context(|| "Failed to run day 3.")?;
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
