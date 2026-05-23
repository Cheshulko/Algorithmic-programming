// https://leetcode.com/problems/check-if-array-is-sorted-and-rotated

class Solution {
   public:
    bool check(vector<int>& nums) {
        const auto ma = *max_element(nums.begin(), nums.end());
        const auto mi = *min_element(nums.begin(), nums.end());
        const auto n = nums.size();

        auto bad = false;
        for (size_t i = 0; i < n; ++i) {
            const auto a = nums[i];
            const auto b = nums[(i + 1) % n];
            if (a > b) {
                if (bad) {
                    return false;
                }
                bad = true;
                if (a != ma || b != mi) {
                    return false;
                }
            }
        }

        return true;
    }
};