use super::files::FileReader;
use super::Day;
use anyhow::Result;
use std::path::Path;

#[derive(PartialEq, Debug)]
struct Range {
    min: usize,
    max: usize,
}

pub struct Day4 {
    pub file_reader: Box<dyn FileReader>,
    pub file_path: &'static Path,
}

impl Default for Day4 {
    fn default() -> Self {
        Self {
            file_reader: Box::new(super::files::FileSystemReader {}),
            file_path: Path::new("assets/day_4/input.txt"),
        }
    }
}

impl Day<(usize, usize)> for Day4 {
    fn run(&self) -> Result<(usize, usize)> {
        let input = self.file_reader.read_file(self.file_path)?;
        let mut total_overlaps = 0;
        let mut partials = 0;
        for line in input {
            let (p1, p2) = make_pairs(line);
            if overlaps(&p1, &p2) {
                total_overlaps += 1;
            }
            if partial_overlaps(&p1, &p2) {
                partials += 1;
            }
        }
        println!("Total overlaps: {}", total_overlaps);
        println!("Partial overlaps: {}", partials);
        Ok((total_overlaps, partials))
    }
}

fn make_pair(range_string: &str) -> Range {
    let (min, max) = range_string.split_once('-').unwrap();
    Range {
        min: min.parse::<usize>().unwrap(),
        max: max.parse::<usize>().unwrap(),
    }
}

fn make_pairs(line: String) -> (Range, Range) {
    let (p1, p2) = line.split_once(',').unwrap();
    (make_pair(p1), make_pair(p2))
}

fn overlaps(p1: &Range, p2: &Range) -> bool {
    if p1.min <= p2.min && p1.max >= p2.max {
        true
    } else if p2.min <= p1.min && p2.max >= p1.max {
        true
    } else {
        false
    }
}

fn partial_overlaps(p1: &Range, p2: &Range) -> bool {
    for i in p1.min..=p1.max {
        if i >= p2.min && i <= p2.max {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::files::tests::MockFileReader;

    #[test]
    fn test_returns_number_of_total_overlapped_pairs() {
        let lines = vec!["1-5,2-3".to_string(), "1-2,4-5".to_string()];
        let day = Day4 {
            file_reader: Box::new(MockFileReader { lines }),
            file_path: Path::new("some-file.txt"),
        };
        let result = day.run().unwrap();

        assert_eq!(result.0, 1);

    }

    #[test]
    fn test_returns_number_of_partial_overlaps() {
        let lines = vec!["5-7,7-9".to_string(), "2-4,6-8".to_string()];
        let day = Day4 {
            file_reader: Box::new(MockFileReader { lines }),
            file_path: Path::new("some-file.txt"),
        };
        let result = day.run().unwrap();

        assert_eq!(result.1, 1);
    }

    #[test]
    fn test_make_pair() {
        let line = "1-5,2-3".to_string();
        let result = make_pairs(line);

        assert_eq!(result.0, Range { min: 1, max: 5 });
        assert_eq!(result.1, Range { min: 2, max: 3 });
    }

    mod overlaps {
        use super::*;

        #[test]
        fn test_returns_true_if_ranges_overlap() {
            let p1 = Range { min: 1, max: 5 };
            let p2 = Range { min: 2, max: 3 };

            let result = overlaps(&p1, &p2);

            assert_eq!(result, true);
        }

        #[test]
        fn test_returns_false_if_ranges_do_not_overlap() {
            let p1 = Range { min: 1, max: 5 };
            let p2 = Range { min: 6, max: 7 };

            let result = overlaps(&p1, &p2);

            assert_eq!(result, false);
        }
    }

    mod partial_overlaps {
        use super::*;

        #[test]
        fn test_returns_true_if_ranges_partially_overlap() {
            let p1 = Range { min: 1, max: 5 };
            let p2 = Range { min: 4, max: 7 };

            let result = partial_overlaps(&p1, &p2);

            assert_eq!(result, true);
        }

        #[test]
        fn test_returns_false_if_ranges_do_not_partially_overlap() {
            let p1 = Range { min: 1, max: 5 };
            let p2 = Range { min: 6, max: 7 };

            let result = partial_overlaps(&p1, &p2);

            assert_eq!(result, false);
        }
    }
}
