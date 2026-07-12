// https://leetcode.com/problems/rank-transform-of-an-array

class Solution {
   public:
    vector<int> arrayRankTransform(vector<int>& arr) {
        const auto n = arr.size();

        vector<pair<int, size_t>> arr_ord;
        arr_ord.reserve(n);
        for (auto i = 0; i < n; ++i) {
            arr_ord.push_back({arr[i], i});
        }
        sort(arr_ord.begin(), arr_ord.end());

        vector<int> ans(n, 0);
        int rank = 0;
        int prev = numeric_limits<int>::min();
        for (auto i = 0; i < n; ++i) {
            if (arr_ord[i].first != prev) {
                prev = arr_ord[i].first;
                ++rank;
            }
            ans[arr_ord[i].second] = rank;
        }

        return ans;
    }
};