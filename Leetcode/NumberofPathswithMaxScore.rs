// https://leetcode.com/problems/number-of-paths-with-max-score

struct Solution;

impl Solution {
    pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
        use std::cmp::Ordering;

        const MOD: usize = 1_000_000_000 + 7;

        let board = board
            .into_iter()
            .map(|row| {
                row.into_bytes()
                    .into_iter()
                    .map(|b| match b {
                        b'0'..=b'9' => (b - b'0') as usize,
                        b'X' => usize::MAX,
                        _ => 0,
                    })
                    .rev()
                    .collect::<Vec<_>>()
            })
            .rev()
            .collect::<Vec<_>>();

        let n = board.len();
        let m = board[0].len();

        let mut dp = vec![vec![(0, 0); m]; n];
        dp[0][0].1 = 1;

        for i in 0..n {
            for j in 0..m {
                for (di, dj) in [(1, 0), (1, 1), (0, 1)] {
                    if board[i][j] == usize::MAX || dp[i][j].1 == 0 {
                        continue;
                    }

                    let (ii, jj) = (di + i, dj + j);
                    if ii == n || jj == m {
                        continue;
                    }
                    if board[ii][jj] == usize::MAX {
                        continue;
                    }

                    let maybe = dp[i][j].0 + board[ii][jj];
                    match dp[ii][jj].0.cmp(&maybe) {
                        Ordering::Less => {
                            dp[ii][jj].0 = maybe;
                            dp[ii][jj].1 = dp[i][j].1;
                        }
                        Ordering::Equal => {
                            dp[ii][jj].1 += dp[i][j].1;
                            dp[ii][jj].1 %= MOD;
                        }
                        Ordering::Greater => {}
                    }
                }
            }
        }

        vec![dp[n - 1][m - 1].0 as i32, dp[n - 1][m - 1].1 as i32]
    }
}
