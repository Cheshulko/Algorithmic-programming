// https://leetcode.com/problems/rotate-image

struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        const N: usize = 4;

        let n = matrix.len();

        for outer in 0..n / 2 {
            let icur = outer;
            let j = outer;

            for jcur in j..n - j - 1 {
                let indxs = [
                    (icur, jcur),
                    (jcur, n - 1 - icur),
                    (n - 1 - icur, n - 1 - jcur),
                    (n - 1 - jcur, icur),
                ];

                let values: [i32; N] = std::array::from_fn(|k| {
                    let (i, j) = indxs[k];
                    matrix[i][j]
                });

                for i in 0..N {
                    matrix[indxs[i].0][indxs[i].1] = values[(i + N - 1) % N];
                }
            }
        }
    }
}
