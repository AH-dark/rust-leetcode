//! https://leetcode.com/problems/string-compression-iii/

use crate::Solution;

impl Solution {
    pub fn compressed_string(word: String) -> String {
        let iter = word.as_bytes().into_iter();
        let mut result: Vec<u8> = vec![];

        let mut byte = 0;
        let mut count = 0;
        for b in iter {
            if byte == 0 {
                byte = *b;
                count = 1;
            } else if byte == *b && count < 9 {
                count += 1;
            } else {
                result.push(b'0' + count);
                result.push(byte);
                byte = *b;
                count = 1;
            }
        }

        if count > 0 {
            result.push(b'0' + count);
            result.push(byte);
        }

        String::from_utf8(result).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compressed_string() {
        assert_eq!(Solution::compressed_string("abcde".to_string()), "1a1b1c1d1e");
        assert_eq!(Solution::compressed_string("aaaaaaaaaaaaaabb".to_string()), "9a5a2b");
    }
}
