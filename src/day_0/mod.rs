use super::Day;
use anyhow::Result;

pub struct Day0;
impl Day<()> for Day0 {
    fn run(&self) -> Result<()> {
        println!("Day 0!");
        Ok(())
    }
}
