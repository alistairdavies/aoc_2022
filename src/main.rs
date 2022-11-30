use clap::{Parser};
use aoc_2022::{day_0};

#[derive(Parser)]
struct Cli {
    day: usize,
}

fn main() {
    let args = Cli::parse();
    run(args.day);
}


fn run(day_number: usize) {
    match day_number {
        0 => day_0::run(),
        _ => println!("Day {} not implemented yet", day_number),
    }
}
