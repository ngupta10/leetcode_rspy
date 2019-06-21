/*
 * Given a 2d grid map of '1's (land) and '0's (water), count the number of islands.
 * An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically.
 * You may assume all four edges of the grid are all surrounded by water.
 *
 * Example 1:
 * ----------
 * Input:
 * 11110
 * 11010
 * 11000
 * 00000
 * Output: 1
 *
 * Example 2:
 * -----------
 * Input:
 * 11000
 * 11000
 * 00100
 * 00011
 * Output: 3
 */

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {

    if grid.len() == 0 {
        return 0
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut islands = 0;

    let mut grid = grid;

    for row_index in 0..rows {
        for col_index in 0..cols {
            if grid[row_index][col_index] == '1' {
                islands += 1;
                dfs(&mut grid, row_index as i32, col_index as i32);
            }
        }
    }

    return islands
}

pub fn dfs(board: &mut Vec<Vec<char>>, row: i32, col: i32) -> i32 {
    if (row < 0 || row >= board.len() as i32) || (col < 0 || col >= board[0].len() as i32) || (board[row as usize][col as usize] == '0') {
        return 0
    }
    board[row as usize][col as usize] = '0';
    dfs(board, row + 1, col);
    dfs(board, row - 1, col);
    dfs(board, row, col + 1);
    dfs(board, row, col - 1);
    return 1
}

#[cfg(test)]
mod test {
    use super::num_islands;

    #[test]
    fn example1() {
        let input = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0']];
        assert_eq!(num_islands(input), 1);
    }

    #[test]
    fn example2() {
        let input = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1']];
        assert_eq!(num_islands(input), 3);
    }
}
