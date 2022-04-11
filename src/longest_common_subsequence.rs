use std::cmp::max;

///La función devuelve una grilla con los largos de subsecuencias

pub fn longest_common_subsequence(lines1: Vec<String>, lines2: Vec<String>) -> Vec<Vec<u8>> {
    let m = lines1.len();
    let n = lines2.len();
    let mut matrix = vec![vec![0;n+1]; m+1];

    for i in 0..m {
        for j in 0..n {
            let c = lines1.get(i);
            let d = lines2.get(j);
            if c == d {
                matrix[i + 1][j + 1] = matrix[i][j] + 1;
            } else {
                matrix[i + 1][j + 1] = max(matrix[i + 1][j], matrix[i][j + 1]);
            }
        }
    }
    matrix
}

#[cfg(test)]
mod longest_common_subsequence_should {
    use super::longest_common_subsequence;

    #[test]
    fn return_all_zero_grid_given_empty_files() {
        let zero_matrix = vec![vec![0;1]; 1];
        assert_eq!(longest_common_subsequence(vec![], vec![]), zero_matrix);
    }

   #[test]
    fn return_longest_subsequence_size() {
        let seq1 = vec![String::from("abc"), String::from("abcd")];
        let seq2 = vec![String::from("abc"), String::from("abcde")];
        let grid = longest_common_subsequence(seq1, seq2);
        assert_eq!(grid[1][1], 1);
    }
}
