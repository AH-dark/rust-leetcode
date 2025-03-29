use crate::Solution;

fn dfs(
    edges: &Vec<i32>,
    si: usize,
    visit: &mut Vec<bool>,
    store: &mut Vec<usize>,
) -> i32 {
    if si == usize::MAX {
        return -1;
    }

    if visit[si] {
        let mut count = -1;
        for (i, &val) in store.iter().enumerate() {
            if val == si {
                count = i as i32;
                break;
            }
        }

        if count == -1 {
            return -1;
        }

        return store.len() as i32 - count;
    }

    visit[si] = true;
    store.push(si);
    dfs(edges, edges[si] as usize, visit, store)
}

impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let mut visit = vec![false; edges.len()];

        let mut max_cycle = -1;
        for i in 0..edges.len() {
            if visit[i] {
                continue;
            }

            let mut store = Vec::new();
            max_cycle = max_cycle.max(dfs(&edges, i, &mut visit, &mut store));
        }

        max_cycle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_cycle() {
        assert_eq!(Solution::longest_cycle(vec![3, 3, 4, 2, 3]), 3);
        assert_eq!(Solution::longest_cycle(vec![2, -1, 3, 1]), -1);

        assert_eq!(Solution::longest_cycle(vec![3, 4, 0, 2, -1, 2]), 3);
        assert_eq!(Solution::longest_cycle(vec![1, 0, 1, 0]), 2);
    }
}
