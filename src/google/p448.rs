/*
 * Given an array of integers where 1 ≤ a[i] ≤ n (n = size of array), some elements appear twice and others appear once.
 *
 * Find all the elements of [1, n] inclusive that do not appear in this array.
 * Could you do it without extra space and in O(n) runtime? You may assume the returned list does not count as extra space.
 *
 * Example:
 * ---------
 * Input: [4, 3, 2, 7, 8, 2, 3, 1]
 * Output: [5, 6]
 */

pub fn find_disappeard_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut zero_vec = vec![0; nums.len()];
    let mut res: Vec<i32> = vec![];

    for num in nums {
        zero_vec[(num-1) as usize] = 1
    }

    for (index, val) in zero_vec.iter().enumerate() {
        if *val == 0 {
            res.push((index + 1) as i32)
        }
    }

    return res
}

#[cfg(test)]
mod test {
    use super::find_disappeard_numbers;

    #[test]
    fn example1() {
        let input = vec![4, 3, 2, 7, 8, 2, 3, 1];
        assert_eq!(find_disappeard_numbers(input), vec![5, 6]);
    }
}
