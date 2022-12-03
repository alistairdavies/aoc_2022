use super::files::FileReader;
use super::Day;
use anyhow::Error;
use anyhow::Result;
use std::path::Path;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub struct Day3 {
    pub file_reader: Box<dyn FileReader>,
    pub file_path: &'static Path,
}

impl Day<usize> for Day3 {
    fn run(&self) -> Result<usize> {
        let input = self.file_reader.read_file(self.file_path)?;
        let mut total = 0;
        for rucksack in input {
            let (c1, c2) = split(&rucksack);
            let duplicate_char = find_duplicate_char(c1, c2).unwrap();
            total += ALPHABET.find(duplicate_char).unwrap() + 1;
        }
        println!("Total: {}", total);
        Ok(total)
    }
}

fn split(rucksack: &str) -> (&str, &str) {
    let halfway = rucksack.len() / 2;
    rucksack.split_at(halfway)
}

fn find_duplicate_char(c1: &str, c2: &str) -> Result<char> {
    for c in c1.chars() {
        if c2.contains(c) {
            return Ok(c);
        }
    }
    Err(Error::msg("No duplicate char found"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::files::tests::MockFileReader;

    #[test]
    fn test_returns_sum_of_duplicate_items() {
        let lines = vec!["abcdbf".to_string(), "ABCDBF".to_string()];
        let day = Day3 {
            file_reader: Box::new(MockFileReader { lines }),
            file_path: Path::new("some-file.txt"),
        };

        let result = day.run().unwrap();

        assert_eq!(result, 30);
    }

    #[test]
    fn test_splits_string_in_half() {
        let input = "abcdbf";
        let (first, second) = split(input);

        assert_eq!(first, "abc");
        assert_eq!(second, "dbf");
    }

    mod find_duplicate_character {
        use super::*;

        #[test]
        fn test_returns_duplicate_character() {
            let c1 = "abc";
            let c2 = "dbf";

            let result = find_duplicate_char(c1, c2).unwrap();

            assert_eq!(result, 'b');
        }

        #[test]
        fn test_returns_error_when_no_duplicate_character() {
            let c1 = "abc";
            let c2 = "def";

            let result = find_duplicate_char(c1, c2);

            assert!(result.is_err());
        }

        #[test]
        fn test_is_case_sensitive() {
            let c1 = "aBc";
            let c2 = "ABC";

            let result = find_duplicate_char(c1, c2).unwrap();

            assert_eq!(result, 'B');
        }
    }
}
