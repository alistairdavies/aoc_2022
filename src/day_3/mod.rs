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

impl Default for Day3 {
    fn default() -> Self {
        Self {
            file_reader: Box::new(super::files::FileSystemReader {}),
            file_path: Path::new("assets/day_3/input.txt"),
        }
    }
}

impl Day<(usize, usize)> for Day3 {
    fn run(&self) -> Result<(usize, usize)> {
        let input = self.file_reader.read_file(self.file_path)?;
        let mut total = 0;
        let mut id_total = 0;
        let mut group = Vec::new();
        for rucksack in input {
            group.push(rucksack.clone());
            if group.len() == 3 {
                id_total += get_identifier_score(&group).unwrap();
                group.clear();
            }
            total += get_compartment_score(&rucksack);
        }
        println!("Total: {}", total);
        println!("Sticker total: {}", id_total);
        Ok((total, id_total))
    }
}

fn get_identifier_score(group: &Vec<String>) -> Result<usize> {
    for c in ALPHABET.chars() {
        if group[0].contains(c) && group[1].contains(c) && group[2].contains(c) {
            return Ok(score_for_char(c));
        }
    }

    Err(Error::msg("No match found in group"))
}

fn get_compartment_score(rucksack: &str) -> usize {
    let (c1, c2) = split(&rucksack);
    let duplicate_char = find_duplicate_char(c1, c2).unwrap();
    score_for_char(duplicate_char)
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

fn score_for_char(c: char) -> usize {
    ALPHABET.find(c).unwrap() + 1
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

        assert_eq!(result.0, 30);
    }

    #[test]
    fn test_returns_sum_of_common_group_items() {
        let lines = vec!["abcdbf".to_string(), "AbCDbF".to_string(), "bb".to_string()];
        let day = Day3 {
            file_reader: Box::new(MockFileReader { lines }),
            file_path: Path::new("some-file.txt"),
        };

        let result = day.run().unwrap();

        assert_eq!(result.1, 2);
    }

    #[test]
    fn test_splits_string_in_half() {
        let input = "abcdbf";
        let (first, second) = split(input);

        assert_eq!(first, "abc");
        assert_eq!(second, "dbf");
    }

    #[test]
    fn test_score_for_char() {
        assert_eq!(score_for_char('a'), 1);
        assert_eq!(score_for_char('A'), 27);
        assert_eq!(score_for_char('Z'), 52);
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
