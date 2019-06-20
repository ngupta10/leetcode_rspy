/*
 * For some fixed N, an array A is beautiful if it is a permutation of the integers 1, 2, ..., N, such that:
 * For every i < j, there is no k with i < k < j such that A[k] * 2 = A[i] + A[j].
 *
 * Given N, return any beautiful array A.  (It is guaranteed that one exists.)
 *
 * Example 1:
 * -----------
 * Input: 4
 * Output: [2, 1, 4, 3]
 *
 * Input: 5
 * Output: [3, 1, 2, 5, 4]
 */

pub fn beautiful_array(n: i32) -> Vec<i32> {

    return vec![0]
}

#[cfg(test)]
mod test {
    use super::beautiful_array;

    #[test]
    fn example1() {
        let input = 4;
        let res = beautiful_array(input);
        let ans = vec![2, 1, 4, 3];
        assert_eq!(res, ans);
    }
}
