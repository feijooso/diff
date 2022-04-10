mod read_file_lines;
mod longest_common_subsequence;
mod diff;

use std::str::Chars;
use crate::longest_common_subsequence::longest_common_subsequence;
use crate::read_file_lines::read_file_lines;
use crate::diff::print_diff;


fn main() {

    let lines =  read_file_lines("/home/sofia/FIUBA/TALLER I/diff/src/example.txt").unwrap();
    let lines2 = read_file_lines("/home/sofia/FIUBA/TALLER I/diff/src/example2.txt").unwrap();

    let lena = lines.len();
    let lenb = lines2.len();


    let grid = longest_common_subsequence(lines.clone(), lines2.clone());
    print_diff(grid, lines, lines2, lena, lenb)
    
}
