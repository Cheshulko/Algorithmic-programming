// https://leetcode.com/problems/length-of-longest-subarray-with-at-most-k-frequency

class Solution {
   public:
    int maxSubarrayLength(vector<int>& nums, int k) {
        const int n = nums.size();

        int ans = 0;
        unordered_map<int, int> freq;

        for (int l = 0, r = 0; r < n; ++r) {
            l = min(l, r);

            ++freq[nums[r]];

            if (freq[nums[r]] <= k) {
                ans = max(ans, r - l + 1);
            } else {
                while (l <= r && freq[nums[r]] > k) {
                    --freq[nums[l]];
                    ++l;
                }
            }
        }

        return ans;
    }
};