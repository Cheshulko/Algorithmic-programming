// https://leetcode.com/problems/maximum-element-after-decreasing-and-rearranging

class Solution {
   public:
    int maximumElementAfterDecrementingAndRearranging(vector<int>& arr) {
        const auto n = arr.size();

        sort(arr.begin(), arr.end());

        arr[0] = 1;
        for (size_t i = 1; i < n; ++i) {
            arr[i] = min(arr[i], arr[i - 1] + 1);
        }

        return arr[n - 1];
    }
};