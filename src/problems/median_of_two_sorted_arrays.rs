//! https://leetcode.com/problems/median-of-two-sorted-arrays/

use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/median-of-two-sorted-arrays/
    pub fn find_median_sorted_arrays(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
    ) -> f64 {
        let (a, b) = if nums1.len() <= nums2.len() {
            (&nums1, &nums2)
        } else {
            (&nums2, &nums1)
        };

        let (m, n) = (a.len(), b.len());
        let mut left = 0;
        let mut right = m;

        while left <= right {
            let partition_a = (left + right) / 2;
            let partition_b = (m + n + 1) / 2 - partition_a;

            let max_left_a = if partition_a == 0 { i32::MIN } else { a[partition_a - 1] };
            let min_right_a = if partition_a == m { i32::MAX } else { a[partition_a] };

            let max_left_b = if partition_b == 0 { i32::MIN } else { b[partition_b - 1] };
            let min_right_b = if partition_b == n { i32::MAX } else { b[partition_b] };

            if max_left_a <= min_right_b && max_left_b <= min_right_a {
                return if (m + n) % 2 == 0 {
                    (max_left_a.max(max_left_b) + min_right_a.min(min_right_b)) as f64 / 2.0
                } else {
                    max_left_a.max(max_left_b) as f64
                };
            } else if max_left_a > min_right_b {
                right = partition_a - 1;
            } else {
                left = partition_a + 1;
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_median_sorted_arrays() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    }
}
