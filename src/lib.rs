pub fn add(left: i64, right: i64) -> i64 {
    left + right
}

pub fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn gcd_of_48_and_18_is_6() {
        assert_eq!(gcd(48, 18), 6);
    }

    #[test]
    fn gcd_of_0_and_5_is_5() {
        assert_eq!(gcd(0, 5), 5);
    }
}
