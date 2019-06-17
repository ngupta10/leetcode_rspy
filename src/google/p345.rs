/*
 * Write a function that takes a string as input and reverse only the vowels of a string.
 *
 * Example 1:
 * -------------
 * Input: "hello"
 * Output: "holle"
 *
 * Example 2:
 * -------------
 * Input: "leetcode"
 * Output: "leotcede"
 */

pub fn reverse_vowels(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();
    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let mut vowel_indices: Vec<usize> = vec![];
    let mut res = chars.clone();

    for (i, c) in chars.iter().enumerate() {
        if vowels.contains(c) {
            vowel_indices.push(i)
        }
    }

    if vowel_indices.is_empty() {
        return s
    }

    let mut index = 0;
    let mut last_index = vowel_indices.len() - 1;
    while last_index != 0 {
        let elem = vowel_indices[index];
        let last = vowel_indices[last_index];
        res[elem] = chars[last];
        res[last] = chars[elem];
        index += 1;
        last_index -= 1;
    }

    return res.into_iter().collect()
}

#[cfg(test)]
mod test {
    use super::reverse_vowels;

    #[test]
    fn example1() {
        let s = String::from("hello");
        assert_eq!(reverse_vowels(s), String::from("holle"));
    }

    #[test]
    fn example2() {
        let s = String::from("leetcode");
        assert_eq!(reverse_vowels(s), String::from("leotcede"));
    }

    #[test]
    fn example3() {
        let s = String::from("hhhh");
        assert_eq!(reverse_vowels(s), String::from("hhhh"));
    }

    #[test]
    fn example4() {
        let s = String::from("ha");
        assert_eq!(reverse_vowels(s), String::from("ha"));
    }

    #[test]
    fn example5() {
        let s = String::from("aha");
        assert_eq!(reverse_vowels(s), String::from("aha"));
    }

    #[test]
    fn example6() {
        let s = String::from("ahaa");
        assert_eq!(reverse_vowels(s), String::from("ahaa"));
    }

    #[test]
    fn example7() {
        let s = String::from("aA");
        assert_eq!(reverse_vowels(s), String::from("Aa"));
    }
}
