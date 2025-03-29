//! https://leetcode.com/problems/course-schedule-ii/

use std::collections::VecDeque;

use crate::Solution;

impl Solution {
    pub fn find_order(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let mut graph = vec![vec![]; num_courses as usize];
        let mut indegree = vec![0; num_courses as usize];

        for pre in prerequisites {
            let (course, depend) = (pre[0], pre[1]);
            graph[depend as usize].push(course);
            indegree[course as usize] += 1;
        }

        let mut queue = VecDeque::new();
        let mut order = vec![];

        for i in 0..num_courses {
            if indegree[i as usize] == 0 {
                queue.push_back(i);
            }
        }

        while let Some(node) = queue.pop_front() {
            order.push(node);
            for &neighbor in &graph[node as usize] {
                indegree[neighbor as usize] -= 1;
                if indegree[neighbor as usize] == 0 {
                    queue.push_back(neighbor);
                }
            }
        }

        if order.len() == num_courses as usize {
            order
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_order() {
        assert_eq!(Solution::find_order(2, vec![vec![1, 0]]), vec![0, 1]);
        assert_eq!(
            Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]),
            vec![0, 1, 2, 3]
        );
        assert_eq!(Solution::find_order(1, vec![]), vec![0])
    }
}
