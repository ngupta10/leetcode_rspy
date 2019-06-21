/*
 * Implement pow(x, n), which calculates x raised to the power n (x^n)
 */

pub fn my_pow(x: f64, n: i32) -> f64 {

    if n == 0 {
        return 1.0
    }

    if x == 0.0 {
        return 0.0
    }

    if x == 1.0 {
        return 1.0
    }

    if x == -1.0 {
        if n % 2 == 0 {
            return 1.0
        } else {
            return -1.0
        }
    }

    if n < 0 {
        let x = 1.0/x;
        match n.checked_mul(-1) {
            None => 0.0,
            Some(n1) => {
                exp_by_sq(x, n1)
            }
        }
    } else {
        exp_by_sq(x, n)
    }
}

fn exp_by_sq(x: f64, n: i32) -> f64 {
    let mut y = 1.0;
    let mut n = n.clone();
    let mut x = x.clone();
    while n > 1 {
        if n % 2 == 0 {
            x = x*x;
            n = n/2;
        } else {
            y = x * y;
            x = x * x;
            n = (n-1)/2;
        }
    }
    return x * y
}

#[cfg(test)]
mod test {
    use super::my_pow;

    #[test]
    fn example1() {
        let res = my_pow(2.00000, 10);
        assert_eq!(res, 1024.00000);
    }

    // #[test]
    // fn example2() {
    //     let res = my_pow(2.1000, 3);
    //     assert_eq!(res, 9.26100);
    // }

    #[test]
    fn example3() {
        let res = my_pow(2.00000, -2);
        assert_eq!(res, 0.25);
    }

    #[test]
    fn example4() {
        let res = my_pow(2.00000, -2147483648);
        assert_eq!(res, 0.0);
    }
}
