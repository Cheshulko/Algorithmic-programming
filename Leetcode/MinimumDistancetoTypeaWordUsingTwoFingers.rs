// https://leetcode.com/problems/minimum-distance-to-type-a-word-using-two-fingers

struct Solution;

impl Solution {
    pub fn minimum_distance(word: String) -> i32 {
        const N: usize = 5;
        const M: usize = 6;
        const GRID: &[&[char; M]; N] = &[
            &['a', 'b', 'c', 'd', 'e', 'f'],
            &['g', 'h', 'i', 'j', 'k', 'l'],
            &['m', 'n', 'o', 'p', 'q', 'r'],
            &['s', 't', 'u', 'v', 'w', 'x'],
            &['y', 'z', '-', '-', '-', '-'],
        ];

        let get_pos = |x: usize| -> (usize, usize) { (x / M, x % M) };
        let set_pos = |(i, j): (usize, usize)| -> usize { i * M + j };
        let find = |c: char| -> (usize, usize) {
            for i in 0..N {
                for j in 0..M {
                    if GRID[i][j] == c {
                        return (i, j);
                    }
                }
            }
            unreachable!()
        };
        let dist_pos = |x: usize, y: usize| -> i32 {
            if x == N * M || x == N * M {
                return 0;
            }
            let (ix, jx) = get_pos(x);
            let (iy, jy) = get_pos(y);
            (ix.abs_diff(iy) + jx.abs_diff(jy)) as i32
        };

        let word = word.to_ascii_lowercase().chars().collect::<Vec<_>>();
        let n = word.len();

        let mut dp = vec![[[i32::MAX; M * N + 1]; M * N + 1]; n + 1];
        dp[1][set_pos(find(word[0]))][N * M] = 0;
        dp[1][N * M][set_pos(find(word[0]))] = 0;

        for l in 2..=n {
            let c = word[l - 1];
            let pos_c = set_pos(find(c));

            for f1 in 0..N * M {
                for f2 in 0..=N * M {
                    if f1 == f2 || dp[l - 1][f1][f2] == i32::MAX {
                        continue;
                    }
                    let d1 = dist_pos(f1, pos_c);
                    dp[l][pos_c][f2] = dp[l][pos_c][f2].min(dp[l - 1][f1][f2] + d1);

                    let d2 = dist_pos(f2, pos_c);
                    dp[l][f1][pos_c] = dp[l][f1][pos_c].min(dp[l - 1][f1][f2] + d2);
                }
            }
        }

        let mut ans = i32::MAX;
        for f1 in 0..N * M {
            for f2 in 0..N * M {
                ans = ans.min(dp[n][f1][f2]);
            }
        }

        ans
    }
}
