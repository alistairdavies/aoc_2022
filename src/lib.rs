pub mod day_0;
pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod files;
use anyhow::Result;

pub trait Day<T> {
    fn run(&self) -> Result<T>;
}
