/*
 * Given a string containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
 *
 * An input string is valid if:
 *      Open brackets must be closed by the same type of brackets.
 *      Open brackets must be closed in the correct order.
 *
 * Note that an empty string is also considered valid.
 *
 * Examples:
 * ----------
 * Input: "()"
 * Output: true
 *
 * Input: "()[]{}"
 * Output: true
 *
 * Input: "(]"
 * Output: false
 *
 * Input: "([)]"
 * Output: false
 *
 * Input: "{[]}"
 * Output: true
 */
use std::collections::HashSet;

pub fn is_valid(s: String) -> bool {
    if s.is_empty() {
        return true
    }

    if s.len() == 1 {
        return false
    }

    let openers: HashSet<char> = ['{', '(', '['].iter().cloned().collect();
    let closers: HashSet<char> = ['}', ')', ']'].iter().cloned().collect();
    let chars: Vec<char> = s.chars().collect();

    if closers.contains(&chars[0]) {
        return false
    }

    let mut stack: Vec<char> = vec![];

    for i in chars {
        if openers.contains(&i) {
            stack.insert(0, i)
        }

        if closers.contains(&i) {
            if !stack.is_empty() {
                if matcher(&stack[0], &i) {
                    stack.remove(0);
                } else {
                    return false
                }
            } else {
                return false
            }
        }
    }

    return stack.len() == 0
}

fn matcher(opener: &char, closer: &char) -> bool {
    if *opener == '{' {
        return *closer == '}'
    }
    if *opener == '(' {
        return *closer == ')'
    }
    if *opener == '[' {
        return *closer == ']'
    }
    return false
}


#[cfg(test)]
mod test {
    use super::is_valid;

    #[test]
    fn example1() {
        let s = String::from("()");
        assert_eq!(is_valid(s), true);
    }

    #[test]
    fn example2() {
        let s = String::from("()[]{}");
        assert_eq!(is_valid(s), true);
    }

    #[test]
    fn example3() {
        let s = String::from("");
        assert_eq!(is_valid(s), true);
    }

    #[test]
    fn example4() {
        let s = String::from("(]");
        assert_eq!(is_valid(s), false);
    }

    #[test]
    fn example5() {
        let s = String::from("([)]");
        assert_eq!(is_valid(s), false);
    }

    #[test]
    fn example6() {
        let s = String::from("{[]}");
        assert_eq!(is_valid(s), true);
    }

    #[test]
    fn example7() {
        let s = String::from("}");
        assert_eq!(is_valid(s), false);
    }

    #[test]
    fn example8() {
        let s = String::from("}[");
        assert_eq!(is_valid(s), false);
    }

    #[test]
    fn example9() {
        let s = String::from(")[]");
        assert_eq!(is_valid(s), false);
    }

    #[test]
    fn example10() {
        let s = String::from("[])");
        assert_eq!(is_valid(s), false);
    }

    #[test]
    fn example11() {
        let s = String::from("(])");
        assert_eq!(is_valid(s), false);
    }
}
