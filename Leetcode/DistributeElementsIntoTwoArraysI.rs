// https://leetcode.com/problems/distribute-elements-into-two-arrays-i

struct Solution;

impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let mut arr1 = vec![nums[0]];
        let mut arr2 = vec![nums[1]];

        for num in nums.into_iter().skip(2) {
            if arr1.last().unwrap() > arr2.last().unwrap() {
                arr1.push(num);
            } else {
                arr2.push(num);
            }
        }

        arr1.extend(arr2);
        arr1
    }
}
