// https://leetcode.com/problems/minimize-hamming-distance-after-swap-operations

struct Solution;

impl Solution {
    pub fn minimum_hamming_distance(
        source: Vec<i32>,
        target: Vec<i32>,
        allowed_swaps: Vec<Vec<i32>>,
    ) -> i32 {
        let n = source.len();
        let allowed_swaps = allowed_swaps.into_iter().fold(vec![vec![]; n], |mut g, p| {
            let u = p[0] as usize;
            let v = p[1] as usize;
            g[v].push(u);
            g[u].push(v);
            g
        });

        fn dfs(cur: usize, allowed_swaps: &[Vec<usize>], seen: &mut [bool], acc: &mut Vec<usize>) {
            seen[cur] = true;
            acc.push(cur);

            for &to in allowed_swaps[cur].iter() {
                if !seen[to] {
                    dfs(to, allowed_swaps, seen, acc);
                }
            }
        }

        let mut seen = vec![false; n];
        let mut ans = 0;
        for i in 0..n {
            if !seen[i] {
                let mut acc = vec![];
                dfs(i, &allowed_swaps, &mut seen, &mut acc);

                let mut s1 = acc.iter().map(|&i| source[i]).collect::<Vec<_>>();
                s1.sort_unstable();

                let mut s2 = acc.iter().map(|&i| target[i]).collect::<Vec<_>>();
                s2.sort_unstable();

                let (mut i, mut j) = (0, 0);
                use std::cmp::Ordering;
                let mut cnt = 0;
                let n = s1.len();
                while i < n && j < n {
                    match s1[i].cmp(&s2[j]) {
                        Ordering::Less => {
                            cnt += 1;
                            i += 1
                        }
                        Ordering::Equal => {
                            i += 1;
                            j += 1
                        }
                        Ordering::Greater => {
                            cnt += 1;
                            j += 1
                        }
                    }
                }
                cnt += n - i;
                cnt += n - j;
                ans += cnt / 2;
            }
        }

        ans as i32
    }
}
