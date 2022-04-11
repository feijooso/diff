/// La función imprime cuáles lineas son diferentes entre dos archivos
/// grid es la grilla computada por longest_common_subsequence
/// a y b son las lineas de los archivos que se quiere comparar
/// i y j son la ubicacion dentro de la grilla

pub fn print_diff(grid: Vec<Vec<u8>>, a: Vec<String>, b: Vec<String>, i: usize, j: usize) {
    if i > 0 && j > 0 && a[i - 1] == b[j - 1] {
        println!("  {}", a[i - 1]);
        print_diff(grid, a, b, i - 1, j - 1);
    } else if j > 0 && (i == 0 || grid[i][j - 1] >= grid[i - 1][j]) {
        println!("> {}", b[j - 1]);
        print_diff(grid, a, b, i, j - 1);
    } else if i > 0 && (j == 0 || grid[i][j - 1] < grid[i - 1][j]) {
        println!("< {}", a[i - 1]);
        print_diff(grid, a, b, i - 1, j);
    } else {
        println!()
    }
}
