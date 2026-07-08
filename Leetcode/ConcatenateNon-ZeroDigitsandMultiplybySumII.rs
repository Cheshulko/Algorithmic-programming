// https://leetcode.com/problems/concatenate-non-zero-digits-and-multiply-by-sum-ii

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
    pub fn sum_and_multiply(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const MOD: usize = 1_000_000_000 + 7;

        let mut ss = vec![];
        let mut sn = vec![];
        for (i, b) in s.into_bytes().into_iter().enumerate() {
            if b == b'0' {
                continue;
            }
            ss.push((b - b'0') as usize);
            sn.push(i);
        }

        if sn.is_empty() {
            return vec![0; queries.len()];
        }

        let mut v = vec![];
        let mut p = 1;
        for &d in ss.iter().rev() {
            v.push(d * p % MOD);
            p = (p * 10) % MOD;
        }
        v.reverse();

        let mut pref_p = vec![0; v.len() + 1];
        for (i, &p) in v.iter().enumerate() {
            pref_p[i + 1] = (pref_p[i] + p) % MOD;
        }

        let mut pref_s = vec![0; ss.len() + 1];
        for (i, &d) in ss.iter().enumerate() {
            pref_s[i + 1] = (pref_s[i] + d) % MOD;
        }

        let mut ans = vec![0; queries.len()];
        for (i, q) in queries.into_iter().enumerate() {
            let [l, r] = [q[0] as usize, q[1] as usize];

            let l = sn.partition_point(|&i| i < l);
            let r = sn.partition_point(|&i| i <= r);
            //[l; r)

            let sum = pref_s[r] - pref_s[l];

            let prod = (MOD + pref_p[r] - pref_p[l]) % MOD;
            let pow = sn.len() - r;
            let den = cm_modular::modular_exponential(10, pow as i64, MOD as i64);
            let den = cm_modular::mod_inverse(den, MOD as i64) as usize;
            let prod = prod * den % MOD;

            ans[i] = (sum * prod % MOD) as i32;
        }

        ans
    }
}
