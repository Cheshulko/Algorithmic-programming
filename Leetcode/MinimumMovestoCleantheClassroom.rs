// https://leetcode.com/problems/minimum-moves-to-clean-the-classroom

struct Solution;

impl Solution {
    pub fn min_moves(classroom: Vec<String>, energy: i32) -> i32 {
        use std::collections::VecDeque;

        let classroom = classroom
            .into_iter()
            .map(|r| r.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let n = classroom.len();
        let m = classroom[0].len();
        let energy = energy as usize;

        let mut ls_cnt = 0;
        let mut ls = vec![vec![0; m]; n];
        let (mut si, mut sj) = (0, 0);
        for i in 0..n {
            for j in 0..m {
                match classroom[i][j] {
                    'S' => {
                        si = i;
                        sj = j;
                    }
                    'L' => {
                        ls[i][j] = ls_cnt + 1;
                        ls_cnt += 1;
                    }
                    _ => {}
                }
            }
        }

        let mut dp = vec![vec![vec![vec![usize::MAX; 1 << ls_cnt]; energy + 1]; m]; n];
        let mut q = VecDeque::new();

        q.push_back((si, sj, 0, energy));
        dp[si][sj][energy][0] = 0;

        const DIRS: &[(i32, i32)] = &[(1, 0), (0, -1), (-1, 0), (0, 1)];

        while let Some((i, j, m, e)) = q.pop_front() {
            if e == 0 {
                continue;
            }

            for (to_i, to_j) in DIRS.iter().filter_map(|(di, dj)| {
                let to_i = (i as i32 + di) as usize;
                let to_j = (j as i32 + dj) as usize;

                (*classroom.get(to_i)?.get(to_j)? != 'X').then_some((to_i, to_j))
            }) {
                let to_m = if ls[to_i][to_j] > 0 {
                    1 << (ls[to_i][to_j] - 1)
                } else {
                    0
                } | m;

                let to_e = if classroom[to_i][to_j] == 'R' {
                    energy
                } else {
                    e - 1
                };

                if dp[to_i][to_j][to_e][to_m] <= dp[i][j][e][m] + 1 {
                    continue;
                }

                dp[to_i][to_j][to_e][to_m] = dp[i][j][e][m] + 1;
                q.push_back((to_i, to_j, to_m, to_e));
            }
        }

        let mut ans = usize::MAX;
        for e in 0..=energy {
            for i in 0..n {
                for j in 0..m {
                    ans = ans.min(dp[i][j][e][(1 << ls_cnt) - 1]);
                }
            }
        }

        if ans == usize::MAX {
            -1
        } else {
            ans as i32
        }
    }
}
