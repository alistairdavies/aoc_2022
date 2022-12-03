use super::files::FileReader;
use super::Day;
use anyhow::{Error, Result};
use std::path::Path;

pub struct Day2 {
    pub file_reader: Box<dyn FileReader>,
    pub file_path: &'static Path,
}

impl Default for Day2 {
    fn default() -> Self {
        Self {
            file_reader: Box::new(super::files::FileSystemReader{}),
            file_path: Path::new("assets/day_2/input.txt"),
        }
    }
}

enum Signals {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Day<usize> for Day2 {
    fn run(&self) -> Result<usize> {
        let input = self.file_reader.read_file(self.file_path)?;
        let mut score = 0;
        for game in input {
            score += get_score(&game);
        }
        println!("Total score: {}", score);
        Ok(score)
    }
}

fn get_score(game: &str) -> usize {
    let (opponent, result) = game.split_once(' ').unwrap();
    let (opponent, result) = (
        opponent.chars().next().unwrap(),
        result.chars().next().unwrap(),
    );
    let signal_score = get_required_signal(opponent, result).unwrap();
    let result_score = get_result_score(result).unwrap();
    println!(
        "{} vs {}: ({} + {}) = {}",
        opponent,
        result,
        signal_score,
        result_score,
        signal_score + result_score
    );

    signal_score + result_score
}

fn get_result_score(me: char) -> Result<usize> {
    match me {
        'X' => Ok(0),
        'Y' => Ok(3),
        'Z' => Ok(6),
        _ => Err(Error::msg("Invalid input")),
    }
}

fn get_required_signal(opponent: char, result: char) -> Result<usize> {
    let required = match (opponent, result) {
        ('A', 'X') => Signals::Scissors,
        ('A', 'Y') => Signals::Rock,
        ('A', 'Z') => Signals::Paper,
        ('B', 'X') => Signals::Rock,
        ('B', 'Y') => Signals::Paper,
        ('B', 'Z') => Signals::Scissors,
        ('C', 'X') => Signals::Paper,
        ('C', 'Y') => Signals::Scissors,
        ('C', 'Z') => Signals::Rock,
        _ => return Err(Error::msg("Invalid signal")),
    };

    Ok(required as usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::files::tests::MockFileReader;

    #[test]
    fn test_returns_sum_of_score() {
        let lines = vec![
            "A Y".to_string(), // 4
            "B X".to_string(), // 1
            "C Z".to_string(), // 7
        ];
        let day = Day2 {
            file_reader: Box::new(MockFileReader { lines }),
            file_path: Path::new("some-file.txt"),
        };

        let result = day.run().unwrap();

        assert_eq!(result, 12);
    }

    mod get_score {
        use super::*;

        #[test]
        fn test_returns_sum_of_result_and_signal() {
            let result = get_score("A Y");
            assert_eq!(result, 4);
        }
    }

    mod get_result_score {
        use super::*;

        #[test]
        fn test_returns_0_for_loss() {
            let result = get_result_score('X').unwrap();
            assert_eq!(result, 0);
        }

        #[test]
        fn test_returns_6_for_win() {
            let result = get_result_score('Z').unwrap();
            assert_eq!(result, 6);
        }

        #[test]
        fn test_returns_3_for_draw() {
            let result = get_result_score('Y').unwrap();
            assert_eq!(result, 3);
        }
    }

    mod get_required_signal {
        use super::*;

        #[test]
        fn test_returns_required_signal_for_loss() {
            let result = get_required_signal('A', 'X').unwrap();
            assert_eq!(result, 3);
        }

        #[test]
        fn test_returns_required_signal_for_win() {
            let result = get_required_signal('A', 'Z').unwrap();
            assert_eq!(result, 2);
        }

        #[test]
        fn test_returns_required_signal_for_draw() {
            let result = get_required_signal('C', 'Y').unwrap();
            assert_eq!(result, 3);
        }
    }
}
