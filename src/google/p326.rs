/*
 * Given an integer, write a function to determine if it is a power of three.
 *
 * Examples:
 * ----------
 * Input: 0
 * Output: false
 *
 * Input: 27
 * Output: true
 *
 * Input: 9
 * Output: true
 *
 * Input: 45
 * Output: false
 */

pub fn is_power_of_three(n: i32) -> bool {
    let mut num = n.clone();

    while num % 3 == 0 && num > 0 {
        num /= 3
    }

    return num == 1

}

#[cfg(test)]
mod test {
    use super::is_power_of_three;

    #[test]
    fn example1() {
        let input = 1;
        assert_eq!(is_power_of_three(input), true);
    }

    #[test]
    fn example2() {
        let input = 0;
        assert_eq!(is_power_of_three(input), false);
    }

    #[test]
    fn example3() {
        let input = 3;
        assert_eq!(is_power_of_three(input), true);
    }

    #[test]
    fn example4() {
        let input = 9;
        assert_eq!(is_power_of_three(input), true);
    }

    #[test]
    fn example5() {
        let input = 27;
        assert_eq!(is_power_of_three(input), true);
    }

    #[test]
    fn example6() {
        let input = 45;
        assert_eq!(is_power_of_three(input), false);
    }
}
