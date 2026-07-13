// https://leetcode.com/problems/sequential-digits

struct Solution;

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut ans = vec![];

        for s in 1..10 {
            let mut num = s;
            while num <= high {
                if num >= low {
                    ans.push(num);
                }
                if num % 10 == 9 {
                    break;
                }
                num = num * 10 + (1 + num % 10);
            }
        }

        ans.sort_unstable();
        ans
    }
}
