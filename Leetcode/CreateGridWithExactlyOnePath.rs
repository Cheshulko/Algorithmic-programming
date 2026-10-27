// https://leetcode.com/problems/create-grid-with-exactly-one-path

struct Solution;

impl Solution {
    pub fn create_grid(m: i32, n: i32) -> Vec<String> {
        let (n, m) = (n as usize, m as usize);

        let mut ans = vec![vec!['#'; n]; m];
        for j in 0..n {
            ans[0][j] = '.';
        }
        for i in 0..m {
            ans[i][n - 1] = '.';
        }

        ans.into_iter().map(|r| r.into_iter().collect()).collect()
    }
}
