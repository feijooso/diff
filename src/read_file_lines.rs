use std::fs::File;
use std::io::{prelude::*, BufReader, Error};

pub fn read_file_lines(file_path: &str) -> Result<Vec<String>, Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    Ok(lines)
}

mod tests {
    use super::*;

    #[test]
    fn should_fail_if_file_doesnt_exist() {
        let file_path = "non-existent-file.txt";

        assert_eq!(read_file_lines(file_path).unwrap_err().to_string(), "No such file or directory (os error 2)");
    }

    #[test]
    fn should_return_lines_in_file() {
        let file_path = "/home/sofia/FIUBA/TALLER I/diff/src/example.txt";
        let lines = ["hola", "chau"];

        assert_eq!(read_file_lines(file_path).unwrap(), lines);
    }
}