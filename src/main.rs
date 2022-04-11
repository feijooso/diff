mod diff;
mod longest_common_subsequence;
mod read_file_lines;
mod grid;

use std::env;

use crate::diff::print_diff;
use crate::longest_common_subsequence::longest_common_subsequence;
use crate::read_file_lines::read_file_lines;

fn main() {
    let arg: Vec<String> = env::args().collect();
    let path = env::current_dir().unwrap();
    let file_path1 = path.join(&arg[1]);
    let file_path2 = path.join(&arg[2]);
    let lines = read_file_lines(file_path1.to_str().unwrap()).unwrap();
    let lines2 = read_file_lines(file_path2.to_str().unwrap()).unwrap();

    let lena = lines.len();
    let lenb = lines2.len();

    let grid = longest_common_subsequence(lines.clone(), lines2.clone());
    print_diff(grid, lines, lines2, lena, lenb)
}
