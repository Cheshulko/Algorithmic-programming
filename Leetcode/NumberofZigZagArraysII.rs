// https://leetcode.com/problems/number-of-zigzag-arrays-ii

pub mod matrix {
    use std::fmt;

    #[derive(Debug, Clone)]
    pub struct Matrix<const MOD: usize> {
        data: Vec<Vec<usize>>,
        rows: usize,
        cols: usize,
    }

    impl<const MOD: usize> fmt::Display for Matrix<MOD> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for row in &self.data {
                write!(f, "(")?;
                for (j, &val) in row.iter().enumerate() {
                    if j > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{:3}", val)?;
                }
                writeln!(f, ")")?;
            }
            Ok(())
        }
    }

    impl<const MOD: usize> Matrix<MOD> {
        pub fn new(data: Vec<Vec<usize>>) -> Self {
            assert!(!data.is_empty());
            assert!(!data[0].is_empty());

            let rows = data.len();
            let cols = data[0].len();

            Matrix { data, rows, cols }
        }

        pub fn identity(n: usize) -> Self {
            let mut data = vec![vec![0; n]; n];
            for i in 0..n {
                data[i][i] = 1;
            }

            Matrix::new(data)
        }

        pub fn multiply(&self, other: &Matrix<MOD>) -> Matrix<MOD> {
            assert_eq!(
                self.cols, other.rows,
                "Matrix dimensions do not match for multiplication"
            );

            let mut result = vec![vec![0; other.cols]; self.rows];
            for i in 0..self.rows {
                for j in 0..other.cols {
                    for k in 0..self.cols {
                        result[i][j] += self.data[i][k] * other.data[k][j];
                        result[i][j] %= MOD;
                    }
                }
            }

            Matrix::new(result)
        }

        pub fn transpose(&self) -> Matrix<MOD> {
            let mut data = vec![vec![0; self.rows]; self.cols];
            for i in 0..self.rows {
                for j in 0..self.cols {
                    data[j][i] = self.data[i][j];
                }
            }
            Matrix::new(data)
        }

        pub fn pow(&self, mut exp: u64) -> Matrix<MOD> {
            assert_eq!(
                self.rows, self.cols,
                "Only square matrices can be exponentiated"
            );

            let mut base = self.clone();
            let mut result = Matrix::identity(self.rows);

            while exp > 0 {
                if exp % 2 == 1 {
                    result = result.multiply(&base);
                }
                base = base.multiply(&base);
                exp /= 2;
            }

            result
        }

        pub fn elemements_sum(&self) -> usize {
            let mut s = 0;
            for row in &self.data {
                for &el in row {
                    s += el;
                    s %= MOD;
                }
            }

            s
        }
    }
}

struct Solution;

impl Solution {
    pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
        const M: usize = 1_000_000_000 + 7;

        let r = (r - l + 1 + 1) as usize;
        let n = n as u64 - 1;

        let mut A = vec![vec![1; r]];
        A[0][0] = 0;
        let A: matrix::Matrix<M> = matrix::Matrix::new(A);
        let B = A.clone();

        let mut L = vec![vec![0; r]; r];
        for i in 1..r {
            for j in 2 + i - 1..r {
                L[i][j] = 1;
            }
        }

        let L: matrix::Matrix<M> = matrix::Matrix::new(L);
        let R = L.transpose();

        let RL = R.multiply(&L);
        let LR = L.multiply(&R);

        let n2 = n / 2;

        let RL_n2 = RL.pow(n2);
        let LR_n2 = LR.pow(n2);

        let mut A = A.multiply(&RL_n2);
        let mut B = B.multiply(&LR_n2);

        if n & 1 > 0 {
            let A2 = B.multiply(&L);
            let B2 = A.multiply(&R);
            A = B2;
            B = A2;
        }

        ((A.elemements_sum() + B.elemements_sum()) % M) as i32
    }
}
