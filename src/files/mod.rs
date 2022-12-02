use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub trait FileReader {
    fn read_file(&self, path: &Path) -> Result<Vec<String>>;
}

pub struct FileSystemReader;
impl FileReader for FileSystemReader {
    fn read_file(&self, path: &Path) -> Result<Vec<String>> {
        let file = File::open(path)?;
        let lines = BufReader::new(file).lines();
        let processed_lines = lines
            .map(|line| line.unwrap().trim_end().to_string())
            .collect();
        Ok(processed_lines)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_file_returns_lines() {
        let mut tmp_file = NamedTempFile::new().unwrap();
        writeln!(tmp_file, "Nice file contents").unwrap();

        let mut content = FileSystemReader {}.read_file(tmp_file.path()).unwrap();

        assert_eq!(content.pop().unwrap(), "Nice file contents");
    }

    #[test]
    fn test_read_file_removes_trailing_whitespace() {
        let mut tmp_file = NamedTempFile::new().unwrap();
        writeln!(tmp_file, "cool   ").unwrap();

        let mut content = FileSystemReader {}.read_file(tmp_file.path()).unwrap();

        assert_eq!(content.pop().unwrap(), "cool");
    }

    pub struct MockFileReader {
        pub lines: Vec<String>,
    }

    impl FileReader for MockFileReader {
        fn read_file(&self, _: &Path) -> Result<Vec<String>> {
            Ok(self.lines.clone())
        }
    }
}
