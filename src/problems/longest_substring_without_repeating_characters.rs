use crate::Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_length = 0;
        let mut start = 0;
        let mut char_index = vec![-1i32; 128];

        for (i, c) in s.chars().enumerate() {
            let index = c as usize;

            start = start.max(char_index[index] + 1);
            char_index[index] = i as i32;
            max_length = max_length.max(i as i32 - start + 1);
        }

        max_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
    }
}
