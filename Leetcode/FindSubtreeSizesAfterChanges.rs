// https://leetcode.com/problems/find-subtree-sizes-after-changes

struct Solution;

impl Solution {
    pub fn find_subtree_sizes(parent: Vec<i32>, s: String) -> Vec<i32> {
        let n = parent.len();
        let s = s
            .into_bytes()
            .into_iter()
            .map(|b| (b - b'a') as usize)
            .collect::<Vec<_>>();

        let adj = parent
            .iter()
            .enumerate()
            .fold(vec![vec![]; n], |mut v, (i, &p)| {
                if i == 0 {
                    v
                } else {
                    v[p as usize].push(i);
                    v
                }
            });

        fn dfs(
            p: i32,
            cur: usize,
            adj: &Vec<Vec<usize>>,
            s: &Vec<usize>,
            seen: &mut [Vec<usize>; 26],
            adj_new: &mut Vec<Vec<usize>>,
        ) {
            if let Some(&last) = seen[s[cur]].last() {
                adj_new[last].push(cur);
            } else if p != -1 {
                adj_new[p as usize].push(cur);
            }

            seen[s[cur]].push(cur);

            for &to in adj[cur].iter() {
                if to as i32 == p {
                    continue;
                }

                dfs(cur as i32, to, adj, s, seen, adj_new);
            }

            seen[s[cur]].pop();
        }

        let mut seen: [_; 26] = std::array::from_fn(|_| Vec::<usize>::new());
        let mut adj_new = vec![vec![]; n];
        dfs(-1, 0, &adj, &s, &mut seen, &mut adj_new);

        fn dfs_cnt(p: i32, cur: usize, adj: &Vec<Vec<usize>>, sz: &mut Vec<i32>) -> i32 {
            let mut size = 1;

            for &to in adj[cur].iter() {
                if to as i32 == p {
                    continue;
                }

                size += dfs_cnt(cur as i32, to, adj, sz);
            }

            sz[cur] = size;
            size
        }

        let mut sz = vec![0; n];
        _ = dfs_cnt(-1, 0, &adj_new, &mut sz);

        sz
    }
}
