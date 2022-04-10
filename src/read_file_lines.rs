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
    use std::env;
    use super::*;

    #[test]
    fn should_fail_if_file_doesnt_exist() {
        let file_path = "non-existent-file.txt";

        assert_eq!(
            read_file_lines(file_path).unwrap_err().to_string(),
            "No such file or directory (os error 2)"
        );
    }

    #[test]
    fn should_return_lines_in_file() {
        let path = env::current_dir().unwrap();
        let file_path = path.join("src/example.txt");
        let lines = ["Abc", "Hjk"];
        println!("path is: {}", file_path.to_str().unwrap());

        assert_eq!(read_file_lines(file_path.to_str().unwrap()).unwrap(), lines);
    }
}
