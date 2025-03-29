//! https://leetcode.com/problems/delete-characters-to-make-fancy-string/

use crate::Solution;

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut result: Vec<u8> = vec![];
        let mut count = 0;
        let mut prev_char = 0_u8;

        for c in s.into_bytes() {
            if c == prev_char {
                count += 1;
            } else {
                count = 1;
            }

            if count < 3 {
                result.push(c);
            }

            prev_char = c;
        }

        String::from_utf8(result).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_fancy_string() {
        assert_eq!(Solution::make_fancy_string("leeetcode".to_string()), "leetcode");
        assert_eq!(Solution::make_fancy_string("aaabaaaa".to_string()), "aabaa");
        assert_eq!(Solution::make_fancy_string("aab".to_string()), "aab");
    }
}
