// https://leetcode.com/problems/check-if-there-is-a-valid-path-in-a-grid

const STREET: &[&[(i32, i32)]] = &[
    &[(-1, 0), (1, 0)],
    &[(0, 1), (0, -1)],
    &[(-1, 0), (0, 1)],
    &[(1, 0), (0, 1)],
    &[(-1, 0), (0, -1)],
    &[(1, 0), (0, -1)],
];

struct Solution;

impl Solution {
    pub fn has_valid_path(mut grid: Vec<Vec<i32>>) -> bool {
        fn dfs((i, j): (usize, usize), grid: &mut [Vec<i32>]) {
            let d = grid[i][j] as usize - 1;
            grid[i][j] = -1;

            let tos = STREET[d]
                .iter()
                .filter_map(|(dj, di)| {
                    let to_i = (i as i32 + di) as usize;
                    let to_j = (j as i32 + dj) as usize;

                    let _ = grid.get(to_i)?.get(to_j)?;

                    Some((to_i, to_j, di, dj))
                })
                .collect::<Vec<_>>();

            for (to_i, to_j, di, dj) in tos.into_iter() {
                if grid[to_i][to_j] == -1 {
                    continue;
                }

                if STREET[grid[to_i][to_j] as usize - 1].contains(&(-dj, -di)) {
                    dfs((to_i, to_j), grid);
                }
            }
        }

        dfs((0, 0), &mut grid);

        let n = grid.len();
        let m = grid[0].len();

        grid[n - 1][m - 1] == -1
    }
}
