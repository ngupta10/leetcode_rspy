/*
 * Given two non-negative integers num1 and num2 represented as string, return the sum of num1 and num2.
 *
 * Note:
 * The length of both num1 and num2 is < 5100.
 * Both num1 and num2 contains only digits 0-9.
 * Both num1 and num2 does not contain any leading zero.
 * You must not use any built-in BigInteger library or convert the inputs to integer directly.
*/

pub fn add_string(num1: String, num2: String) -> String {
    let (p1, p2) = pad_inputs(num1, num2);

    let mut index = p1.len();
    let mut carry = 0;
    let mut res: Vec<u32> = vec![];

    while index > 0 {
        let a = p1[index-1].to_digit(10).unwrap_or(0);
        let b = p2[index-1].to_digit(10).unwrap_or(0);
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
    let ans = res.into_iter().map(|i| i.to_string()).collect::<String>();

    return ans
}

fn pad_inputs(num1: String, num2: String) -> (Vec<char>, Vec<char>) {
    let mut chars1: Vec<char> = num1.chars().collect();
    let mut chars2: Vec<char> = num2.chars().collect();
    let mut p1;
    let mut p2;

    let len1 = chars1.len();
    let len2 = chars2.len();

    if len1 > len2 {
        let pad = len1 - len2;
        p2 = vec!['0'; pad];
        p2.append(&mut chars2);
        p1 = chars1;
    } else if len1 < len2 {
        let pad = len2 - len1;
        p1 = vec!['0'; pad];
        p1.append(&mut chars1);
        p2 = chars2;
    } else {
        p1 = chars1;
        p2 = chars2;
    }
    return (p1, p2)
}

#[cfg(test)]
mod test {
    use super::add_string;

    #[test]
    fn example1() {
        let num1 = String::from("1234");
        let num2 = String::from("567");
        assert_eq!(add_string(num1, num2), String::from("1801"));
    }

    #[test]
    fn example2() {
        let num1 = String::from("123");
        let num2 = String::from("5678");
        assert_eq!(add_string(num1, num2), String::from("5801"));
    }

    #[test]
    fn example3() {
        let num1 = String::from("1234");
        let num2 = String::from("5678");
        assert_eq!(add_string(num1, num2), String::from("6912"));
    }

    #[test]
    fn example4() {
        let num1 = String::from("6913259244");
        let num2 = String::from("71103343");
        assert_eq!(add_string(num1, num2), String::from("6984362587"));
    }

    #[test]
    fn example5() {
        let num1 = String::from("401716832807512840963");
        let num2 = String::from("167141802233061013023557397451289113296441069");
        assert_eq!(add_string(num1, num2), String::from("167141802233061013023557799168121920809282032"));
    }

    #[test]
    fn example6() {
        let num1 = String::from("1");
        let num2 = String::from("9");
        assert_eq!(add_string(num1, num2), String::from("10"));
    }

}
