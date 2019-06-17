/*
 * Given a non-empty array of digits representing a non-negative integer, plus one to the integer.
 *
 * The digits are stored such that the most significant digit is at the head of the list,
 * and each element in the array contain a single digit.
 *
 * You may assume the integer does not contain any leading zero, except the number 0 itself.
 *
 * Examples:
 * ----------
 * Input: [1, 2, 3]
 * Output: [1, 2, 4]
 *
 * Input: [4, 3, 2, 1]
 * Output: [4, 3, 2, 2]
 */

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let adder = one_vector(&digits);
    let mut index = digits.len();
    let mut carry = 0;
    let mut res: Vec<i32> = vec![];

    while index > 0 {
        let a = digits[index-1];
        let b = adder[index-1];
        let tmp_sum = a + b + carry;

        if tmp_sum >= 10 {
            let val = tmp_sum % 10;
            carry = 1;
            res.push(val);
        } else {
            let val = tmp_sum;
            carry = 0;
            res.push(val);
        }

        index -= 1;

        if index == 0 && carry == 1 {
            res.push(carry)
        }
    }

    res.reverse();

    return res
}

fn one_vector(digits: &Vec<i32>) -> Vec<i32> {
    let len = digits.len();
    let mut adder = vec![0; len-1];
    adder.push(1);
    return adder
}

#[cfg(test)]
mod test {
    use super::plus_one;

    #[test]
    fn example1() {
        let input = vec![1, 2, 3];
        assert_eq!(plus_one(input), vec![1, 2, 4]);
    }

    #[test]
    fn example2() {
        let input = vec![4, 3, 2, 1];
        assert_eq!(plus_one(input), vec![4, 3, 2, 2]);
    }
}
