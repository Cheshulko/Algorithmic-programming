// https://leetcode.com/problems/sorted-gcd-pair-queries

struct Solution;

impl Solution {
    pub fn gcd_values(nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
        let n = nums.len();
        let ma = nums.iter().max().copied().unwrap() as usize;

        let mut divs = vec![vec![]; n];
        for (i, num) in nums.into_iter().enumerate() {
            let num = num as usize;
            for div in 1.. {
                if div * div > num {
                    break;
                }
                if num % div == 0 {
                    divs[i].push(div);
                    if div * div != num {
                        divs[i].push(num / div);
                    }
                }
            }
        }

        let mut cnt = vec![0; ma + 1];
        for divs in divs.iter() {
            for &div in divs {
                cnt[div] += 1;
            }
        }

        let mut paired_cnt = vec![0; ma + 1];
        for (i, &c) in cnt.iter().enumerate() {
            paired_cnt[i] = c * (c - 1) / 2;
        }

        for num in (2..=ma).rev() {
            let cnt_pairs = paired_cnt[num];

            for div in 1.. {
                if div * div > num {
                    break;
                }
                if num % div == 0 {
                    paired_cnt[div] -= cnt_pairs;
                    if div * div != num && div != 1 {
                        paired_cnt[num / div] -= cnt_pairs;
                    }
                }
            }
        }

        let mut pref: Vec<i64> = vec![0; ma + 1];
        for num in 1..ma + 1 {
            pref[num] = pref[num - 1] + paired_cnt[num];
        }

        queries
            .into_iter()
            .map(|q| pref.partition_point(|&p| p < q + 1) as i32)
            .collect()
    }
}
