/*
 * Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
 *
 * For example, given n = 3, a solution set is:
 * [
  "((()))",
  "(()())",
  "(())()",
  "()(())",
  "()()()"
  ]
 */

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    return vec![String::from("hi")]
}

#[cfg(test)]
mod test {
    use super::generate_parenthesis;

    #[test]
    fn example1() {
        let input = 3;
        let res = vec![
            String::from("((()))"),
            String::from("(()())"),
            String::from("(())()"),
            String::from("()(())"),
            String::from("()()()")];
        assert_eq!(generate_parenthesis(input), res);
    }
}
