use super::files::FileReader;
use super::Day;
use anyhow::Result;
use std::path::Path;

pub struct Day6 {
    pub file_reader: Box<dyn FileReader>,
    pub file_path: &'static Path,
}

impl Default for Day6 {
    fn default() -> Self {
        Self {
            file_reader: Box::new(super::files::FileSystemReader {}),
            file_path: Path::new("assets/day_6/input.txt"),
        }
    }
}

impl Day<usize> for Day6 {
    fn run(&self) -> Result<usize> {
        let input = self.file_reader.read_file(self.file_path)?;
        let signal = input.first().unwrap();
        let mut four_slice = "notunique";
        let mut i = 0;
        while !unique(four_slice) {
            four_slice = &signal[i..i+4];
            i+=1;
        }

        println!("{} is unique", four_slice);
        println!("Seen at char {}", i+3);
        Ok(i+3)
    }
}

fn unique(s: &str) -> bool {
    for c in s.chars() {
        if s.matches(c).count() > 1 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::files::tests::MockFileReader;

    #[test]
    fn test_returns_index_of_start_sequence() {
        let lines = vec!["bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()];
        let day = Day6 {
            file_reader: Box::new(MockFileReader { lines }),
            file_path: Path::new("some-file.txt"),
        };

        let result = day.run().unwrap();

        assert_eq!(result, 5);
    }

    #[test]
    fn test_unique() {
        assert_eq!(unique("abcd"), true);
        assert_eq!(unique("abbb"), false);
        assert_eq!(unique("bvwb"), false);
    }
}
