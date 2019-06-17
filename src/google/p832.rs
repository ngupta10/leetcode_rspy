/*
 * Given a binary matrix A, we want to flip the image horizontally, then invert it, and return the resulting image.
 *
 * To flip an image horizontally means that each row of the image is reversed.
 * For example, flipping [1, 1, 0] horizontally results in [0, 1, 1].
 *
 * To invert an image means that each 0 is replaced by 1, and each 1 is replaced by 0.
 * For example, inverting [0, 1, 1] results in [1, 0, 0].
 *
 * Example 1:
 * ------------
 * Input: [[1, 1, 0], [1, 0, 1], [0, 0, 0]]
 * Output: [[1, 0, 0], [0, 1, 0], [1, 1, 1]]
 *
 * Example 2:
 * -------------
 * Input: [[1, 1, 0, 0], [1, 0, 0, 1], [0, 1, 1, 1], [1, 0, 1, 0]]
 * Output: [[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 1], [1, 0, 1, 0]]
 */

pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut flipped: Vec<Vec<i32>> = vec![];
    let mut res: Vec<Vec<i32>> = vec![];
    for row in a {
        flipped.push(flip_row(row))
    }

    for row in flipped {
        res.push(invert_row(row))
    }

    return res
}

fn flip_row(mut row: Vec<i32>) -> Vec<i32> {
    row.reverse();
    return row
}

fn invert_row(mut row: Vec<i32>) -> Vec<i32> {
    for (i, num) in row.clone().iter().enumerate() {
        row[i] = 1 - num
    }
    return row
}

#[cfg(test)]
mod test {
    use super::flip_and_invert_image;

    #[test]
    fn example1() {
        let input =  vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]];
        let res = flip_and_invert_image(input);
        let ans = vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
        assert_eq!(res, ans);
    }

    #[test]
    fn example2() {
        let input =  vec![vec![1, 1, 0, 0], vec![1, 0, 0, 1], vec![0, 1, 1, 1], vec![1, 0, 1, 0]];
        let res = flip_and_invert_image(input);
        let ans =  vec![vec![1, 1, 0, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 1], vec![1, 0, 1, 0]];
        assert_eq!(res, ans);
    }
}
