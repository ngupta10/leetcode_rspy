/*
 * Given a collection of intervals, merge all overlapping intervals.
 *
 * Example 1:
 * -----------
 * Input: [[1,3],[2,6],[8,10],[15,18]]
 * Output: [[1,6],[8,10],[15,18]]
 * Explanation: Since intervals [1,3] and [2,6] overlaps, merge them into [1,6].
 *
 * Example 2:
 * -----------
 * Input: [[1,4],[4,5]]
 * Output: [[1,5]]
 * Explanation: Intervals [1,4] and [4,5] are considered overlapping.
 */
use std::cmp::Ordering;

pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

    if intervals.is_empty() {
        return vec![];
    }

    let mut intervals = intervals;
    intervals.sort_by(|a, b| sort_by_first(&a, &b));

    let mut stack: Vec<Vec<i32>> = vec![];
    let first = intervals[0].clone();
    stack.push(first);

    for i in 1..intervals.len() {
        let top = stack[(stack.len() - 1)].clone();
        let interval = intervals[i].clone();
        if top[1] < intervals[i][0] {
            stack.push(interval);
        } else if top[1] < intervals[i][1] {
            let new_top = vec![top[0], intervals[i][1]];
            stack.pop();
            stack.push(new_top);
        }
    }

    return stack

}

pub fn sort_by_first(a: &Vec<i32>, b: &Vec<i32>) -> Ordering {
    if a[0] < b[0] {
        return Ordering::Less;
    } else if a[0] == b[0] {
        return Ordering::Equal;
    }
    Ordering::Greater
}

#[cfg(test)]
mod test {
    use super::merge;

    #[test]
    fn example1() {
        let input = vec![vec![2,6], vec![8,10], vec![1, 3], vec![15,18]];
        assert_eq!(merge(input), vec![vec![1,6], vec![8,10], vec![15,18]]);
    }

    #[test]
    fn example2() {
        let input = vec![vec![1,4], vec![4,5]];
        assert_eq!(merge(input), vec![vec![1, 5]]);
    }
}
