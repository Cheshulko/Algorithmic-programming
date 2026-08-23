// https://leetcode.com/problems/sum-game

struct Solution;

impl Solution {
    pub fn sum_game(num: String) -> bool {
        let num = num
            .into_bytes()
            .into_iter()
            .map(|b| {
                if b == b'?' {
                    i32::MAX
                } else {
                    (b - b'0') as i32
                }
            })
            .collect::<Vec<_>>();

        let n = num.len();

        let sum1 = num
            .iter()
            .take(n / 2)
            .filter(|&n| *n != i32::MAX)
            .sum::<i32>();
        let have1 = num.iter().take(n / 2).filter(|&n| *n == i32::MAX).count() as i32;

        let sum2 = num
            .iter()
            .skip(n / 2)
            .filter(|&n| *n != i32::MAX)
            .sum::<i32>();
        let have2 = num.iter().skip(n / 2).filter(|&n| *n == i32::MAX).count() as i32;

        let (have1, have2) = (have1 - have1.min(have2), have2 - have1.min(have2));

        use std::cmp::Ordering;
        match sum1.cmp(&sum2) {
            Ordering::Equal if have1 == have2 => false,
            Ordering::Equal => true,

            _ => {
                if sum1 >= sum2 && have1 > 0 {
                    true
                } else if sum1 <= sum2 && have2 > 0 {
                    true
                } else {
                    if have1 % 2 == 1 || have2 % 2 == 1 {
                        true
                    } else if sum1 - sum2 > 0 && (sum1 - sum2) * 2 == have2 * 9 {
                        false
                    } else if sum2 - sum1 > 0 && (sum2 - sum1) * 2 == have1 * 9 {
                        false
                    } else {
                        true
                    }
                }
            }
        }
    }
}
