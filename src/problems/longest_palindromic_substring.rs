//! https://leetcode.com/problems/longest-palindromic-substring/

use crate::Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() <= 1 {
            return s;
        }

        let t: Vec<char> = {
            let mut t = Vec::with_capacity(s.len() * 2 + 1);
            t.push('#');
            for c in s.chars() {
                t.push(c);
                t.push('#');
            }
            t
        };
        let n = t.len();

        let mut dp = vec![0; n];
        let mut center = 0;
        let mut right = 0;
        let mut max_len = 0;
        let mut max_center = 0;

        for i in 0..n {
            if i < right {
                dp[i] = (right - i).min(dp[2 * center - i]);
            }

            while i - dp[i] > 0 && i + dp[i] + 1 < n && t[i - dp[i] - 1] == t[i + dp[i] + 1] {
                dp[i] += 1;
            }

            if i + dp[i] > right {
                center = i;
                right = i + dp[i];
            }

            if dp[i] > max_len {
                max_len = dp[i];
                max_center = i;
            }
        }

        let start = max_center - max_len;
        let end = max_center + max_len;
        t[start..=end].iter().filter(|c| **c != '#').collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        {
            let res = Solution::longest_palindrome("babad".to_string());
            assert!(res == "bab" || res == "aba");
        }
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
    }
}
