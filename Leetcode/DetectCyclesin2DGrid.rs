// https://leetcode.com/problems/detect-cycles-in-2d-grid

const DIRS: &[(i32, i32)] = &[(1, 0), (0, -1), (-1, 0), (0, 1)];

struct Solution;

impl Solution {
    pub fn contains_cycle(mut grid: Vec<Vec<char>>) -> bool {
        fn dfs(
            cur @ (i, j): (usize, usize),
            prev: (usize, usize),
            c: char,
            grid: &mut [Vec<char>],
        ) -> bool {
            grid[i][j] = (c as u8 ^ b' ') as char;

            let tos = DIRS
                .iter()
                .filter_map(|(di, dj)| {
                    let to_i = (i as i32 + di) as usize;
                    let to_j = (j as i32 + dj) as usize;

                    let _ = grid.get(to_i)?.get(to_j)?;

                    Some((to_i, to_j))
                })
                .collect::<Vec<_>>();

            for (to_i, to_j) in tos.into_iter() {
                if (to_i, to_j) == prev {
                    continue;
                }

                if grid[to_i][to_j] == (c as u8 ^ b' ') as char {
                    return true;
                }

                if grid[to_i][to_j] == c {
                    if dfs((to_i, to_j), cur, c, grid) {
                        return true;
                    }
                }
            }

            false
        }

        let n = grid.len();
        let m = grid[0].len();
        for i in 0..n {
            for j in 0..m {
                if grid[i][j].is_lowercase() {
                    if dfs((i, j), (i, j), grid[i][j], &mut grid) {
                        return true;
                    }
                }
            }
        }

        false
    }
}
