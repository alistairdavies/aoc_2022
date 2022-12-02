use super::files::FileReader;
use super::Day;
use anyhow::Result;
use std::path::Path;

pub struct Day1 {
    pub file_reader: Box<dyn FileReader>,
    pub file_path: &'static Path,
}

impl Day<usize> for Day1 {
    fn run(&self) -> Result<usize> {
        let input = self.file_reader.read_file(self.file_path)?;
        let total_calories = process_input(input);
        let top_three = top_three(&total_calories);
        println!("Top three: {:?}", top_three);
        Ok(top_three.iter().sum())
    }
}

fn process_input(input: Vec<String>) -> Vec<usize> {
    let mut total_calories = vec![];
    let mut current_calories = 0;
    for line in input {
        if line == "" {
            total_calories.push(current_calories);
            current_calories = 0;
        } else {
            current_calories += line.parse::<usize>().unwrap();
        }
    }
    total_calories.push(current_calories);
    total_calories
}

fn top_three(numbers: &Vec<usize>) -> Vec<usize> {
    let mut numbers = numbers.clone();
    numbers.sort();
    numbers.reverse();
    numbers[..3].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::files::tests::MockFileReader;

    #[test]
    fn test_returns_sum_of_top_3_calories() {
        let lines = vec![
            "10".to_string(),
            "20".to_string(),
            "".to_string(),
            "10".to_string(),
            "".to_string(),
            "30".to_string(),
            "".to_string(),
            "23".to_string(),
        ];
        let day = Day1 {
            file_reader: Box::new(MockFileReader { lines }),
            file_path: Path::new("some-file.txt"),
        };

        let result = day.run().unwrap();

        assert_eq!(result, 83);
    }

    #[test]
    fn test_process_input() {
        let input = vec![
            "10".to_string(),
            "20".to_string(),
            "".to_string(),
            "10".to_string(),
        ];

        let result = process_input(input);

        assert_eq!(result, vec![30, 10]);
    }

    #[test]
    fn test_returns_top_three() {
        let numbers = vec![1, 2, 3, 222, 4, 5, 388];

        let result = top_three(&numbers);

        assert_eq!(result, vec![388, 222, 5]);
    }
}
