// https://leetcode.com/problems/cyclically-rotating-a-grid

struct Solution;

impl Solution {
    pub fn rotate_grid(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        fn rotate(grid: &mut Vec<Vec<i32>>, lev: usize, k: usize) {
            let n = grid.len();
            let m = grid[0].len();

            let mut nums = vec![];
            let mut indxs = vec![];

            {
                for j in lev..m - lev {
                    indxs.push((lev, j));
                    nums.push(grid[lev][j]);
                }
            }
            {
                for i in lev + 1..n - lev {
                    indxs.push((i, m - lev - 1));
                    nums.push(grid[i][m - lev - 1]);
                }
            }
            {
                for j in (lev..m - lev - 1).rev() {
                    indxs.push((n - lev - 1, j));
                    nums.push(grid[n - lev - 1][j]);
                }
            }
            {
                for i in (lev + 1..n - lev - 1).rev() {
                    indxs.push((i, lev));
                    nums.push(grid[i][lev]);
                }
            }

            for (ind, (i, j)) in indxs.into_iter().enumerate() {
                grid[i][j] = nums[(ind + k) % nums.len()]
            }
        }

        let k = k as usize;

        let n = grid.len();
        let m = grid[0].len();

        for lev in 0..n.min(m) / 2 {
            rotate(&mut grid, lev, k);
        }

        grid
    }
}
