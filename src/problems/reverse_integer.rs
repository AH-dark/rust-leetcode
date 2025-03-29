use crate::Solution;

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        // remove 0 from the end
        while x % 10 == 0 && x != 0 {
            x /= 10;
        }

        let mut result = 0;
        let mut sign = 1;
        if x < 0 {
            sign = -1;
            x = -x;
        }

        while x > 0 {
            let pop = x % 10;
            x /= 10;

            if result > i32::MAX / 10 || (result == i32::MAX / 10 && pop > 7) {
                return 0; // Return 0 if overflow occurs
            }

            result = result * 10 + pop;
        }

        result * sign
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
    }
}
