//! https://leetcode.com/problems/course-schedule/

use crate::Solution;

impl Solution {
    pub fn can_finish(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
    ) -> bool {
        let mut indegree = vec![0; num_courses as usize];
        let mut graph = vec![vec![]; num_courses as usize];

        for pre in prerequisites {
            let (course, depend) = (pre[0], pre[1]);
            indegree[course as usize] += 1;
            graph[depend as usize].push(course);
        }

        let mut queue = std::collections::VecDeque::new();
        for i in 0..num_courses {
            if indegree[i as usize] == 0 {
                queue.push_back(i);
            }
        }

        let mut count = 0;
        while let Some(node) = queue.pop_front() {
            count += 1;
            for &neighbor in &graph[node as usize] {
                indegree[neighbor as usize] -= 1;
                if indegree[neighbor as usize] == 0 {
                    queue.push_back(neighbor);
                }
            }
        }

        count == num_courses
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_finish() {
        assert!(Solution::can_finish(2, vec![vec![1, 0]]));
        assert!(!Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]));
    }
}
