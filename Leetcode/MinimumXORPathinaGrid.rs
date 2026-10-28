// https://leetcode.com/problems/minimum-xor-path-in-a-grid

struct Solution;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid.len(), grid[0].len());

        let mut dp = vec![vec![[false; 1 << 10]; m + 1]; n + 1];
        dp[1][0][0] = true;
        dp[0][1][0] = true;

        for i in 1..=n {
            for j in 1..=m {
                for x in 0..1 << 10 {
                    let g = grid[i - 1][j - 1] as usize;
                    if dp[i - 1][j][x] || dp[i][j - 1][x] {
                        dp[i][j][g ^ x] = true;
                    }
                }
            }
        }

        dp[n][m]
            .iter()
            .enumerate()
            .filter_map(|(i, &can)| can.then_some(i))
            .next()
            .unwrap() as i32
    }
}
