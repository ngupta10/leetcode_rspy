/*
 * Given a list of daily temperatures T, return a list such that, for each day in the input,
 * tells you how many days you would have to wait until a warmer temperature. If there is no future day for which this is possible, put 0 instead.
 *
 * For example, given the list of temperatures T = [73, 74, 75, 71, 69, 72, 76, 73], your output should be [1, 1, 4, 2, 1, 1, 0, 0].
 * Note: The length of temperatures will be in the range [1, 30000]. Each temperature will be an integer in the range [30, 100].
 */

pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    let mut res: Vec<i32> = vec![];
    while i < t.len() {
        let mut j = i+1;
        while j != t.len() {
            if t[j] > t[i] {
                res.push((j - i) as i32);
                break;
            }
            j += 1
        }
        if j == t.len() {
            res.push(0)
        }
        i += 1
    }

    return res
}


#[cfg(test)]
mod test {
    use super::daily_temperatures;

    #[test]
    fn example1() {
        let input =  vec![73, 74, 75, 71, 69, 72, 76, 73];
        let res = daily_temperatures(input);
        let ans = vec![1, 1, 4, 2, 1, 1, 0, 0];
        assert_eq!(res, ans);
    }
}
