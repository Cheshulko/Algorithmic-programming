// https://leetcode.com/problems/partition-array-according-to-given-pivot

class Solution {
   public:
    vector<int> pivotArray(vector<int>& nums, int pivot) {
        const auto n = nums.size();

        vector<int> extra;
        size_t j = 0;
        size_t p = 0;
        for (auto i = 0; i < n; ++i) {
            if (nums[i] > pivot) {
                extra.push_back(nums[i]);
            } else if (nums[i] == pivot) {
                ++p;
            } else {
                nums[j++] = nums[i];
            }
        }

        while (p--) {
            nums[j++] = pivot;
        }

        for (const auto x : extra) {
            nums[j++] = x;
        }

        return nums;
    }
};