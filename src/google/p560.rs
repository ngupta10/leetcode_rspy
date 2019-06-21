/*
 * Given an array of integers and an integer k,
 * you need to find the total number of continuous subarrays whose sum equals to k.
 *
 * Example 1:
 * ----------
 * Input: nums = [1, 1, 1], k = 2
 * Output: 2
 */
use std::collections::HashMap;

pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {

    let mut count = 0;
    let mut sum = 0;
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(0, 1);

    for i in nums {
        sum += i;
        match map.get(&(sum - k)) {
            None => (),
            Some(v) => {
                count += v
            }
        }
        match map.get(&sum) {
            None => {
                map.insert(sum, 1);
            },
            Some(v) => {
                map.insert(sum, v + 1);
            }
        }
    }

    return count
}


#[cfg(test)]
mod test {
    use super::subarray_sum;

    #[test]
    fn example1() {
        let nums = vec![1, 1, 1];
        let k = 2;
        assert_eq!(subarray_sum(nums, k), 2);
    }
}
