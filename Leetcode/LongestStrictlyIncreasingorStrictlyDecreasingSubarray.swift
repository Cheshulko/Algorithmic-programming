// https://leetcode.com/problems/longest-strictly-increasing-or-strictly-decreasing-subarray

class Solution {
    func longestMonotonicSubarray(_ nums: [Int]) -> Int {
        func solve(_ nums: ArraySlice<Int>, _ cmp: (Int, Int) -> Bool) -> Int {
            guard var prev = nums.first else {
                return 0
            }

            var len = 1
            for num in nums.dropFirst() {
                if cmp(prev, num) {
                    len += 1
                    prev = num
                } else {
                    break
                }
            }

            return len
        }

        let n = nums.count
        var ans = 0;
        for i in 0..<n {
            ans = max(ans, solve(nums[i..<n], <), solve(nums[i..<n], >))
        }

        return ans
    }
}