use std::cmp::max;
use grid::Grid;

pub fn longest_common_subsequence(a: Vec<String>, b: Vec<String>) -> Grid<u8> {
    let m = a.len();
    let n = b.len();
    let mut grid: Grid<u8> = Grid::new(m+1, n+1);

    for i in 0..m+1 {
        grid[i][0] = 0;
    }
    for j in 0..n+1 {
        grid[j][0] = 0;
    }
    for i in 0..m {
        for j in 0..n {
            let c = a.get(i);
            let d = b.get(j);
            if c == d {
                grid[i+1][j+1] = grid[i][j] + 1
            } else {
                grid[i+1][j+1] = max(grid[i+1][j], grid[i][j+1])
            }
        }
    }
    grid
}

mod longest_common_subsequence_should {
    use super::*;

    #[test]
    fn return_all_zero_grid_given_empty_files() {
        let zero_grid = Grid::new(1,1);
        assert_eq!(longest_common_subsequence(vec![], vec![]), zero_grid);
    }

    #[test]
    fn return_longest_subsequence_size() {
        let seq1 = vec![String::from("abc"), String::from("abcd")];
        let seq2 = vec![String::from("abc"), String::from("abcde")];
        let grid = longest_common_subsequence(seq1, seq2);
        assert_eq!(grid[1][1], 1);
    }
}