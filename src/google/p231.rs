/*
 * Given an integer, write a function to determine if it is a power of two.
 *
 * Examples:
 * ----------
 * Input: 1
 * Output: true
 *
 * Input: 16
 * Output: true
 *
 * Input: 218
 * Output: false
 */

pub fn is_power_of_two(n: i32) -> bool {
    if n == 1 {
        return true
    }

    if n <= 0 {
        return false
    }

    let mut modder = 2;
    let mut num = n.abs().clone();
    let mut modders = vec![modder];

    while num > modder {
        if (num % modder) == 0 {
            num /= modder;
        } else {
            return false
        }
        modder *= 2;
        modders.push(modder);
    }

    let rem = num % modder;
    if modders.contains(&rem) || rem == 0 {
        return true
    }

    return false

}

#[cfg(test)]
mod test {
    use super::is_power_of_two;

    #[test]
    fn example1() {
        let input = 1;
        assert_eq!(is_power_of_two(input), true);
    }

    #[test]
    fn example2() {
        let input = 16;
        assert_eq!(is_power_of_two(input), true);
    }

    #[test]
    fn example3() {
        let input = 3;
        assert_eq!(is_power_of_two(input), false);
    }

    #[test]
    fn example4() {
        let input = 218;
        assert_eq!(is_power_of_two(input), false);
    }

    #[test]
    fn example5() {
        let input = 32;
        assert_eq!(is_power_of_two(input), true);
    }

    #[test]
    fn example6() {
        let input = 100000;
        assert_eq!(is_power_of_two(input), false);
    }

    #[test]
    fn example7() {
        let input = 2048;
        assert_eq!(is_power_of_two(input), true);
    }

    #[test]
    fn example8() {
        let input = 6;
        assert_eq!(is_power_of_two(input), false);
    }

    #[test]
    fn example9() {
        let input = 9;
        assert_eq!(is_power_of_two(input), false);
    }

    #[test]
    fn example10() {
        let input = 64;
        assert_eq!(is_power_of_two(input), true);
    }

    #[test]
    fn example11() {
        let input = 48;
        assert_eq!(is_power_of_two(input), false);
    }

    #[test]
    fn example12() {
        let input = 256;
        assert_eq!(is_power_of_two(input), true);
    }

    #[test]
    fn example13() {
        let input = -16;
        assert_eq!(is_power_of_two(input), false);
    }
}
