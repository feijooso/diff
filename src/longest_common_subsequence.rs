use std::cmp::max;
use grid::Grid;

fn longest_common_subsequence(a: &str, b: &str) -> Grid<u8> {
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
            let c = a.chars().nth(i).unwrap();
            let d = b.chars().nth(j).unwrap();
            if c == d {
                grid[i+1][j+1] = grid[i][j] + 1
            } else {
                grid[i+1][j+1] = max(grid[i+1][j], grid[i][j+1])
            }
        }
    }
    return grid;
}

mod should {
    use super::*;

    #[test]
    fn return_all_zero_grid_given_empty_strings() {
        let zero_grid = Grid::new(1,1);
        assert_eq!(longest_common_subsequence("", ""), zero_grid);
    }

    #[test]
    fn return_longest_subsequence_size() {
        let grid = longest_common_subsequence("abcdaf", "acbcf");
        assert_eq!(grid[6][5], 4);
    }
}