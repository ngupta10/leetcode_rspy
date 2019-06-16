/*
 * Given a string, find the first non-repeating character in it and return it's index. If it doesn't exist, return -1.
 *
 * Examples:
 * ----------
 * s = "leetcode"
 * return 0
 *
 * s = "loveleetcode"
 * return 2
 */
use std::collections::HashMap;

pub fn first_uniq_char(s: String) -> i32 {
    let mut freq: HashMap<char, u32> = HashMap::new();
    let char_vec: Vec<char> = s.chars().collect();
    for c in &char_vec {
        *freq.entry(*c).or_insert(0) += 1;
    }

    for (index, c) in char_vec.iter().enumerate() {
        if freq.get(c) == Some(&1) {
            return index as i32
        }
    }

    return -1
}

#[cfg(test)]
mod test {
    use super::first_uniq_char;

    #[test]
    fn example1() {
        let s = String::from("leetcode");
        assert_eq!(first_uniq_char(s), 0);
    }

    #[test]
    fn example2() {
        let s = String::from("loveleetcode");
        assert_eq!(first_uniq_char(s), 2);
    }

    #[test]
    fn example3() {
        let s = String::from("aabbcc");
        assert_eq!(first_uniq_char(s), -1);
    }

    #[test]
    fn example4() {
        let s = String::from("aadadaad");
        assert_eq!(first_uniq_char(s), -1);
    }
}
