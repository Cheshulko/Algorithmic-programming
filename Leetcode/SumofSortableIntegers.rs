// https://leetcode.com/problems/sum-of-sortable-integers

struct Solution;

impl Solution {
    pub fn sortable_integers(nums: Vec<i32>) -> i32 {
        fn check_block(block: &[i32], block_sz: usize) -> (bool, i32, i32) {
            assert!(block_sz == block.len());

            if block_sz == 1 {
                return (true, block[0], block[0]);
            }

            let mut mi = i32::MAX;
            let mut ma = i32::MIN;
            for &x in block.iter() {
                mi = mi.min(x);
                ma = ma.max(x);
            }

            let mut cnt = 0;
            let mut bad = 0;
            for (i, w) in block.windows(2).enumerate() {
                let &[a, b] = w else { panic!() };
                let good = b >= a;
                if !good {
                    bad = i;
                    cnt += 1;
                }
            }

            match cnt {
                0 => (true, mi, ma),
                1 => {
                    if block[0] >= block[block_sz - 1] && block[bad] == ma && block[bad + 1] == mi {
                        (true, mi, ma)
                    } else {
                        (false, mi, ma)
                    }
                }
                _ => (false, mi, ma),
            }
        }

        fn check(i: usize, nums: &[i32]) -> usize {
            let n = nums.len();

            let mut ok = true;
            let mut prev_ma = i32::MIN;
            for j in 0..n / i {
                let (block_ok, block_mi, block_ma) = check_block(&nums[i * j..(i * (j + 1))], i);
                ok &= block_ok;
                ok &= prev_ma <= block_mi;
                prev_ma = block_ma;
                if !ok {
                    break;
                }
            }
            if ok {
                i
            } else {
                0
            }
        }

        let n = nums.len();
        let mut ans = 0;
        for i in 1.. {
            if i * i > n {
                break;
            }
            if n % i != 0 {
                continue;
            }

            ans += check(i, &nums);
            if i != n / i {
                ans += check(n / i, &nums);
            }
        }

        ans as i32
    }
}
