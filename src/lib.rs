pub mod day_0;
pub mod day_1;
pub mod files;
use anyhow::Result;

pub trait Day<T> {
    fn run(&self) -> Result<T>;
}
