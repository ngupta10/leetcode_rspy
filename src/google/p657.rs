/*
 * There is a robot starting at position (0, 0), the origin, on a 2D plane.
 * Given a sequence of its moves, judge if this robot ends up at (0, 0) after it completes its moves.
 *
 * The move sequence is represented by a string, and the character moves[i] represents its ith move.
 * Valid moves are R (right), L (left), U (up), and D (down).
 * If the robot returns to the origin after it finishes all of its moves, return true. Otherwise, return false.
 *
 * Note: The way that the robot is "facing" is irrelevant.
 * "R" will always make the robot move to the right once, "L" will always make it move left, etc.
 * Also, assume that the magnitude of the robot's movement is the same for each move.
 *
 * Example1:
 * -------------
 * Input: "UD"
 * Output: true
 * Explanation: The robot moves up once, and then down once.
 * All moves have the same magnitude, so it ended up at the origin where it started. Therefore, we return true.
 *
 * Example 2:
 * -------------
 * Input: "LL"
 * Output: false
 * Explanation: The robot moves left twice. It ends up two "moves" to the left of the origin.
 * We return false because it is not at the origin at the end of its moves.
 */
use std::collections::HashMap;

pub fn judge_circle(moves: String) -> bool {
    let mut freq: HashMap<char, u32> = HashMap::new();
    let char_vec: Vec<char> = moves.chars().collect();
    for c in char_vec {
        *freq.entry(c).or_insert(0) += 1;
    }

    let count_r = freq.get(&'R');
    let count_l = freq.get(&'L');
    let count_d = freq.get(&'D');
    let count_u = freq.get(&'U');

    if (count_r == count_l) & (count_d == count_u) {
        return true
    }

    return false
}

#[cfg(test)]
mod test {
    use super::judge_circle;

    #[test]
    fn example1() {
        let res = judge_circle(String::from("LL"));
        assert_eq!(res, false);
    }

    #[test]
    fn example2() {
        let res = judge_circle(String::from("UD"));
        assert_eq!(res, true);
    }
}
