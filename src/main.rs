mod diff;
mod longest_common_subsequence;
mod read_file_lines;

use std::env;
use std::path::PathBuf;

use crate::diff::print_diff;
use crate::longest_common_subsequence::longest_common_subsequence;
use crate::read_file_lines::read_file_lines;

///La función main lee el nombre de los archivos a comparar por consola, genera el path e imprime el diff

fn main() {
    let arg: Vec<String> = env::args().collect();
    let file_path1 = get_file_path(&arg[1]);
    let file_path2 = get_file_path(&arg[2]);
    let lines = read_and_unwrap_file(file_path1);
    let lines2 = read_and_unwrap_file(file_path2);

    let lena = lines.len();
    let lenb = lines2.len();

    let grid = longest_common_subsequence(lines.clone(), lines2.clone());
    print_diff(grid, lines, lines2, lena, lenb)
}

fn get_file_path(file: &str) -> PathBuf {
    let path = env::current_dir().unwrap();
    path.join(file)
}

fn read_and_unwrap_file(file_path: PathBuf) -> Vec<String> {
    read_file_lines(file_path.to_str().unwrap()).unwrap()
}
