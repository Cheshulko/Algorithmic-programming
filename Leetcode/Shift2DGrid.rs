// https://leetcode.com/problems/shift-2d-grid

struct Solution;

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let n = grid.len();
        let m = grid[0].len();

        let mut ans = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                let flat = m * i + j;
                let flat = (flat + k) % (n * m);
                let (ii, jj) = (flat / m, flat % m);
                assert!(ii < n && jj < m);

                ans[ii][jj] = grid[i][j];
            }
        }

        ans
    }
}
