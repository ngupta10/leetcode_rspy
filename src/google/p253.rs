/*
 * Given an array of meeting time intervals consisting of start and end times [[s1,e1],[s2,e2],...] (si < ei),
 * find the minimum number of conference rooms required.
 *
 * Example 1:
 * ----------
 * Input: [[0, 30],[5, 10],[15, 20]]
 * Output: 2
 *
 * Example 2:
 * -----------
 * Input: [[7,10],[2,4]]
 * Output: 1
 */

pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
    let n = intervals.len();

    if n == 1 {
        return 1;
    }

    if n == 0 {
        return 0;
    }

    let mut starts = intervals.iter().map(|i| i[0]).collect::<Vec<i32>>();
    let mut ends = intervals.iter().map(|i| i[1]).collect::<Vec<i32>>();

    starts.sort();
    ends.sort();

    let mut rooms_needed = 1;
    let mut result = 1;
    let mut i = 1;
    let mut j = 0;

    while i < n && j < n {
        if starts[i] < ends[j] {
            rooms_needed += 1;
            i += 1;
            if rooms_needed > result {
                result = rooms_needed;
            }
        } else {
            rooms_needed -= 1;
            j += 1
        }
    }

    return result
}

#[cfg(test)]
mod test {
    use super::min_meeting_rooms;

    #[test]
    fn example1() {
        let input = vec![vec![0, 30], vec![5, 10], vec![15, 20]];
        assert_eq!(min_meeting_rooms(input), 2);
    }

    #[test]
    fn example2() {
        let input = vec![vec![7,10], vec![2,4]];
        assert_eq!(min_meeting_rooms(input), 1);
    }
}
