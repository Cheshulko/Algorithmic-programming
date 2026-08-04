// https://leetcode.com/problems/find-missing-elements

struct Solution;

impl Solution {
    pub fn find_missing_elements(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();

        let mut cur = nums[0];
        let mut ans = vec![];
        for x in nums {
            while cur < x {
                ans.push(cur);
                cur += 1;
            }
            cur += 1;
        }

        ans
    }
}
