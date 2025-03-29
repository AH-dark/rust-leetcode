//! https://leetcode.com/problems/minimum-height-trees

use crate::Solution;

impl Solution {
    pub fn find_min_height_trees(
        n: i32,
        edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let mut graph = vec![vec![]; n as usize];
        let mut indegree = vec![0; n as usize];

        for edge in edges {
            let (u, v) = (edge[0], edge[1]);
            graph[u as usize].push(v);
            graph[v as usize].push(u);
            indegree[u as usize] += 1;
            indegree[v as usize] += 1;
        }

        fn remove_leaves(
            leaves: Vec<i32>,
            remaining_nodes: i32,
            graph: &Vec<Vec<i32>>,
            indegree: &mut Vec<i32>,
        ) -> Vec<i32> {
            if remaining_nodes <= 2 {
                return leaves;
            }

            let next_remaining_nodes = remaining_nodes - leaves.len() as i32;

            let new_leaves = leaves
                .into_iter()
                .flat_map(|leaf| {
                    graph[leaf as usize]
                        .iter()
                        .filter_map(|&neighbor| {
                            indegree[neighbor as usize] -= 1;
                            if indegree[neighbor as usize] == 1 {
                                Some(neighbor)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect();

            remove_leaves(new_leaves, next_remaining_nodes, graph, indegree)
        }

        remove_leaves(
            (0..n).filter(|&i| indegree[i as usize] <= 1).collect(),
            n,
            &graph,
            &mut indegree,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min_height_trees() {
        assert_eq!(
            Solution::find_min_height_trees(4, vec![vec![1, 0], vec![1, 2], vec![1, 3]]),
            vec![1]
        );
        assert_eq!(
            Solution::find_min_height_trees(6, vec![vec![0, 3], vec![1, 3], vec![2, 3], vec![3, 4], vec![5, 4]]),
            vec![3, 4]
        );
    }
}
