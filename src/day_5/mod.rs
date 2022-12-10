use super::files::FileReader;
use super::Day;
use anyhow::Result;
use std::path::Path;
use regex::Regex;

pub struct Day5 {
    pub file_reader: Box<dyn FileReader>,
    pub file_path: &'static Path,
}

impl Default for Day5 {
    fn default() -> Self {
        Self {
            file_reader: Box::new(super::files::FileSystemReader {}),
            file_path: Path::new("assets/day_5/input.txt"),
        }
    }
}

impl Day<String> for Day5 {
    fn run(&self) -> Result<String> {
        let input = self.file_reader.read_file(self.file_path)?;
        let (stacks, instructions) = Parser::split_input(input);
        let mut stacks = Parser::parse_stacks(stacks);
        let instructions = Parser::parse_instructions(instructions);
        execute_instructions(&mut stacks, instructions);
        let top_string = tops(stacks);
        println!("{}", top_string);
        Ok(top_string)
    }
}

#[derive(Debug, PartialEq)]
struct Instruction {
    pub quantity: usize,
    pub from: usize,
    pub to: usize,
}

struct Parser {}


fn execute_instructions(stacks: &mut Vec<Vec<char>>, instructions: Vec<Instruction>) {
    for instruction in instructions {
        let from_stack = stacks.get_mut(instruction.from).unwrap();
        let mut moved = from_stack.split_off(from_stack.len() - instruction.quantity);
        let to_stack = stacks.get_mut(instruction.to).unwrap();
        moved.reverse();
        to_stack.extend(moved);
    }
}

fn tops(stacks: Vec<Vec<char>>) -> String {
    let mut top_string = String::new();
    for stack in stacks {
        if let Some(top) = stack.last() {
            top_string.push(*top);
        }
    }
    top_string
}

impl Parser {
    fn split_input(input: Vec<String>) -> (Vec<String>, Vec<String>) {
        let mut stacks = Vec::new();
        let mut instructions = Vec::new();
        for line in input {
            if line.contains('[') {
                stacks.push(line);
            } else if !line.is_empty() {
                instructions.push(line);
            }
        }

        instructions.remove(0);
        (stacks, instructions)
    }

    fn parse_stacks(input: Vec<String>) -> Vec<Vec<char>> {
        let length = input[0].len();
        let mut result = vec![Vec::new(); length/3];
        for line in input {
            let mut i = 1;
            while i < length {
                let letter = line.chars().nth(i).unwrap();
                if letter != ' ' {
                    if let Some(stack) = result.get_mut(i / 4) {
                        stack.insert(0, letter);
                    }
                }
                i += 4;
            }
        }

        result
    }

    fn parse_instructions(input: Vec<String>) -> Vec<Instruction> {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)$").unwrap();
        let mut instructions = Vec::new();
        for line in input {
            let caps = re.captures(&line).unwrap();
            instructions.push(Instruction {
                quantity: caps[1].parse().unwrap(),
                from: caps[2].parse::<usize>().unwrap() - 1, 
                to: caps[3].parse::<usize>().unwrap() - 1, 
            });
        }

       instructions 
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::files::tests::MockFileReader;

    #[test]
    fn test_returns_top_of_stacks() {
        let lines = vec![
            "    [D]    ".to_string(),
            "[N] [C]    ".to_string(),
            "[Z] [M] [P]".to_string(),
            " 1   2   3".to_string(),
            "".to_string(),
            "move 1 from 2 to 1".to_string(),
            "move 3 from 1 to 3".to_string(),
            "move 2 from 2 to 1".to_string(),
            "move 1 from 1 to 2".to_string(),
        ];
        let day = Day5 {
            file_reader: Box::new(MockFileReader { lines }),
            file_path: Path::new("some-file.txt"),
        };
        let result = day.run().unwrap();

        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_execute_instructions() {
        let mut stacks = vec![
            vec!['Z', 'N'],
            vec!['D', 'C', 'M'],
        ];
        let instructions = vec![
            Instruction {
                quantity: 3,
                from: 1,
                to: 0,
            },
        ];
        execute_instructions(&mut stacks, instructions);
        assert_eq!(stacks, vec![
            vec!['Z', 'N', 'M', 'C', 'D'],
            vec![],
        ]);
    }

    #[test]
    fn test_tops() {
        let stacks = vec![
            vec!['Z', 'N', 'M'],
            vec!['D', 'C'],
        ];
        let result = tops(stacks);
        assert_eq!(result, "MC");
    }


    #[test]
    fn test_tops_empty() {
        let stacks = vec![
            vec!['Z', 'N', 'M'],
            vec![],
        ];
        let result = tops(stacks);
        assert_eq!(result, "M");
    }

    mod test_parser {
        use super::*;

        #[test]
        fn test_split_input_lines() {
            let lines = vec![
                "[D]        ".to_string(),
                "[N] [C]    ".to_string(),
                " 1   2   3".to_string(),
                "".to_string(),
                "move 1 from 2 to 1".to_string(),
                "move 3 from 1 to 3".to_string(),
            ];

            let result = Parser::split_input(lines);

            assert_eq!(
                result.0,
                vec!["[D]        ".to_string(), "[N] [C]    ".to_string(),]
            );

            assert_eq!(
                result.1,
                vec![
                    "move 1 from 2 to 1".to_string(),
                    "move 3 from 1 to 3".to_string(),
                ]
            );
        }

        #[test]
        fn test_parse_stacks() {
            let stacks = vec![
                "    [D]    ".to_string(),
                "[N] [C]    ".to_string(),
                "[Z] [M] [P]".to_string(),
            ];

            let result = Parser::parse_stacks(stacks);

            assert_eq!(
                result,
                vec![
                    Vec::from(['Z', 'N']),
                    Vec::from(['M', 'C', 'D']),
                    Vec::from(['P']),
                ]
            );
        }

        #[test]
        fn test_parse_instructions() {
            let instructions = vec![
                "move 1 from 2 to 1".to_string(),
                "move 3 from 1 to 3".to_string(),
            ];

            let result = Parser::parse_instructions(instructions);

            assert_eq!(
                result,
                vec![
                    Instruction {
                        quantity: 1,
                        from: 1,
                        to: 0,
                    },
                    Instruction {
                        quantity: 3,
                        from: 0,
                        to: 2,
                    },
                ]
            );
        }
    }
}
