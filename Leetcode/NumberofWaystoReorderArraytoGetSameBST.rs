// https://leetcode.com/problems/number-of-ways-to-reorder-array-to-get-same-bst

mod cm_modular {
    pub fn gcd_extended(a: i64, m: i64) -> (i64, i64, i64) {
        if a == 0 {
            (m, 0, 1)
        } else {
            let (gcd, y, x) = gcd_extended(m % a, a);
            let z = x - (m / a) * y;
            (gcd, z, y)
        }
    }

    pub fn mod_inverse(mut x: i64, m: i64) -> i64 {
        let mut res = 1;
        let mut exp = m - 2; // Fermat's little theorem

        // Binary exponentiation b ^ (m - 2)
        while exp != 0 {
            if exp % 2 == 1 {
                res = res * x % m;
            }
            x = x * x % m;
            exp /= 2;
        }
        res
    }

    pub fn modular_exponential(base: i64, mut power: i64, modulus: i64) -> i64 {
        if modulus == 1 {
            return 0;
        }

        let mut base = if power < 0 {
            mod_inverse(base, modulus)
        } else {
            base % modulus
        };

        let mut result = 1;
        power = power.abs();

        // Binary exponentiation
        while power > 0 {
            if power & 1 == 1 {
                result = (result * base) % modulus;
            }
            power >>= 1;
            base = (base * base) % modulus;
        }
        result
    }
}

struct Solution;

impl Solution {
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().map(|x| x as usize).collect::<Vec<_>>();

        let n = nums.len();
        let root = nums[0];

        let mut tree: Vec<(Option<usize>, Option<usize>)> = vec![(None, None); n + 1];

        // build
        for x in nums.into_iter().skip(1) {
            let mut cur = root;
            loop {
                if x < cur {
                    if let Some(left) = tree[cur].0 {
                        cur = left;
                    } else {
                        tree[cur].0 = Some(x);
                        break;
                    }
                } else {
                    if let Some(right) = tree[cur].1 {
                        cur = right;
                    } else {
                        tree[cur].1 = Some(x);
                        break;
                    }
                }
            }
        }

        const M: i64 = 1_000_000_000 + 7;

        let mut fac = vec![1; n + 1];
        for i in 2..=n {
            fac[i] = (fac[i - 1] * i as i64) % M;
        }

        fn solve(
            cur: Option<usize>,
            tree: &Vec<(Option<usize>, Option<usize>)>,
            fac: &Vec<i64>,
            dp: &mut Vec<i64>,
            sz: &mut Vec<usize>,
        ) {
            let Some(cur) = cur else {
                return;
            };

            solve(tree[cur].0.clone(), tree, fac, dp, sz);
            solve(tree[cur].1.clone(), tree, fac, dp, sz);

            sz[cur] = 1;
            if let Some(left) = tree[cur].0 {
                sz[cur] += sz[left];
            }
            if let Some(right) = tree[cur].1 {
                sz[cur] += sz[right];
            }

            dp[cur] = fac[sz[cur] - 1];
            if let Some(left) = tree[cur].0 {
                let sz_left_inv = cm_modular::mod_inverse(fac[sz[left]] as i64, M);
                dp[cur] = (dp[cur] * sz_left_inv) % M;
                dp[cur] = (dp[cur] * dp[left]) % M;
            }

            if let Some(right) = tree[cur].1 {
                let sz_right_inv = cm_modular::mod_inverse(fac[sz[right]], M);
                dp[cur] = (dp[cur] * sz_right_inv) % M;
                dp[cur] = (dp[cur] * dp[right]) % M;
            }
        }

        let mut dp = vec![0; n + 1];
        let mut sz = vec![0; n + 1];
        solve(Some(root), &tree, &fac, &mut dp, &mut sz);

        dp[root] as i32 - 1
    }
}
