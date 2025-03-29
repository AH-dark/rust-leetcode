use crate::Solution;

impl Solution {
    pub fn minimum_total_distance(mut robot: Vec<i32>, mut factory: Vec<Vec<i32>>) -> i64 {
        let mut factory = factory
            .into_iter()
            .map(|v| (v[0], v[1]))
            .collect::<Vec<_>>();

        let n = robot.len();
        let m = factory.len();

        robot.sort_unstable();
        factory.sort_unstable_by_key(|(f, _)| *f);

        // Dynamic programming table
        // `dp[i][j]` represents the minimum distance to move `j` robots using the first `i` factories.
        let mut dp = vec![vec![i64::MAX / 2; n + 1]; m + 1];

        for fac in &mut dp {
            fac[0] = 0;
        }

        for i in 1..=m {
            let (pos, limit) = (factory[i - 1].0, factory[i - 1].1 as usize);

            for j in 0..=n {
                dp[i][j] = dp[i - 1][j];

                let mut dist = 0;
                for k in 1..=limit.min(j) {
                    dist += (robot[j - k] - pos).abs() as i64;
                    dp[i][j] = dp[i][j].min(dp[i - 1][j - k] + dist);
                }
            }
        }

        dp[m][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_total_distance() {
        assert_eq!(
            Solution::minimum_total_distance(vec![0, 4, 6], vec![vec![2, 2], vec![6, 2]]),
            4
        );
        assert_eq!(
            Solution::minimum_total_distance(vec![1, -1], vec![vec![-2, 1], vec![2, 1]]),
            2
        );
    }
}
