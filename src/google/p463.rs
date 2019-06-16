/*
 * You are given a map in form of a two-dimensional integer grid where 1 represents land and 0 represents water.
 *
 * Grid cells are connected horizontally/vertically (not diagonally).
 * The grid is completely surrounded by water, and there is exactly one island (i.e., one or more connected land cells).
 *
 * The island doesn't have "lakes" (water inside that isn't connected to the water around the island).
 * One cell is a square with side length 1. The grid is rectangular, width and height don't exceed 100.
 * Determine the perimeter of the island.
 *
 * Example:
 * -------------
 * Input: [[0,1,0,0], [1,1,1,0], [0,1,0,0], [1,1,0,0]]
 * Output: 16
*/

pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut perimeter = 0;

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == 1 {
                // left
                if (col == 0) || (grid[row][col-1] == 0) {
                    perimeter += 1
                }
                // right
                if (col == cols -1) || (grid[row][col+1] == 0) {
                    perimeter += 1
                }
                // up
                if (row == 0) || (grid[row-1][col] == 0) {
                    perimeter += 1
                }
                // down
                if (row == rows -1) || (grid[row+1][col] == 0) {
                    perimeter += 1
                }
            }
        }
    }

    return perimeter
}

#[cfg(test)]
mod test {
    use super::island_perimeter;

    #[test]
    fn example1() {
        let input = vec![vec![0, 1, 0, 0], vec![1, 1, 1, 0], vec![0, 1, 0, 0], vec![1, 1, 0, 0]];
        assert_eq!(island_perimeter(input), 16);
    }

    #[test]
    fn example2() {
        let input = vec![vec![0, 1, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]];
        assert_eq!(island_perimeter(input), 4);
    }
}
