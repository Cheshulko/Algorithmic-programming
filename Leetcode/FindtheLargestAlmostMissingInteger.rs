// https://leetcode.com/problems/find-the-largest-almost-missing-integer

struct Solution;

impl Solution {
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = nums.len();

        let ma = nums.iter().max().copied().unwrap() as usize;
        let (left, right) = (nums[0] as usize, nums[n - 1] as usize);

        let pos = nums
            .into_iter()
            .enumerate()
            .fold(vec![vec![]; ma + 1], |mut pos, (i, num)| {
                pos[num as usize].push(i);
                pos
            });

        if n == k {
            ma as i32
        } else if k == 1 {
            pos.into_iter()
                .enumerate()
                .filter_map(|(i, pos)| (pos.len() == 1).then_some(i as i32))
                .max()
                .unwrap_or(-1)
        } else {
            pos.into_iter()
                .enumerate()
                .filter_map(|(i, pos)| {
                    (pos.len() == 1 && (i == left || i == right)).then_some(i as i32)
                })
                .max()
                .unwrap_or(-1)
        }
    }
}
