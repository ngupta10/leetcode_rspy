/*
 * Given an array of integers, return indices of the two numbers such that they add up to a specific target.
 * You may assume that each input would have exactly one solution, and you may not use the same element twice.
 *
 * Example:
 * ----------
 * Given nums = [2, 7, 11, 15], target = 9,
 * Because nums[0] + nums[1] = 2 + 7 = 9,
 * return [0, 1].
 */

// Single pass using hashtable


use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

    let mut res: Vec<i32> = vec![];

    let mut table: HashMap<i32, usize> = HashMap::new();
    let mut i = 0;
    for num in nums {

        let complement = target - num;

        match table.get(&complement) {
            None => {
                table.insert(num, i);
            }
            Some(v) => {
                // println!("num: {:?}, i: {:?}, v: {:?}, complement: {:?}, res: {:?}, table: {:?}", num, i, v, complement, res, table);
                if *v != i {
                    res.push(*v as i32);
                    res.push(i as i32);
                    return res;
                }

            }
        }

        i += 1;
    }
    return res
}

#[cfg(test)]
mod test {
    use super::two_sum;

    #[test]
    fn example1() {
        let nums = vec![2, 7 , 11, 15];
        let target = 9;
        assert_eq!(two_sum(nums, target), vec![0, 1]);
    }

    #[test]
    fn example2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        assert_eq!(two_sum(nums, target), vec![1, 2]);
    }

    #[test]
    fn example3() {
        let nums = vec![3, 3];
        let target = 6;
        assert_eq!(two_sum(nums, target), vec![0, 1]);
    }
}
