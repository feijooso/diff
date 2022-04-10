use grid::Grid;

// C es la grilla computada por lcs()
// X e Y son las secuencias
// i y j especifican la ubicacion dentro de C que se quiere buscar cuando
//    se lee el diff. Al llamar a estar funcion inicialmente, pasarle
//    i=len(X) y j=len(Y)

pub fn print_diff(grid: Grid<u8>, a: Vec<String>, b: Vec<String>, i: usize, j: usize) {
    if i > 0 && j > 0 && a[i-1] == b[j-1] {
        println!("  {}", a[i-1]);
        print_diff(grid, a, b, i-1, j-1);
    }
    else if j > 0 && (i == 0 || grid[i][j-1] >= grid[i-1][j]) {
        println!("> {}", b[j-1]);
        print_diff(grid, a, b, i, j-1);
    }
    else if i > 0 && (j == 0 || grid[i][j-1] < grid[i-1][j]) {
        println!("< {}", a[i-1]);
        print_diff(grid, a, b, i-1, j);
    }
    else { println!("") }
}

mod diff_should {
    use super::*;
    use crate::longest_common_subsequence::longest_common_subsequence;

    #[test]
    fn print_diff_for_given_sequences() {
        let grid = longest_common_subsequence("123", "143");
        let seq1 = vec![&'1',&'2',&'3'];
        let seq2 = vec![&'1',&'4',&'3'];
        let lena = seq1.len();
        let lenb = seq1.len();

        print_diff(grid, seq1, seq2, lena, lenb);

        assert_eq!(1,1);
    }
}