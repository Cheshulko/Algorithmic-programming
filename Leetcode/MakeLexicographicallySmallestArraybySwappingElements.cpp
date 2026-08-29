// https://leetcode.com/problems/make-lexicographically-smallest-array-by-swapping-elements

class Solution {
   public:
    vector<int> lexicographicallySmallestArray(vector<int>& nums, int limit) {
        const auto n = nums.size();

        vector<int> ids(n);
        iota(ids.begin(), ids.end(), 0);

        sort(ids.begin(), ids.end(),
             [&](const int a, const int b) { return nums[a] < nums[b]; });

        vector<vector<int>> groups(1, vector<int>());
        int prev = nums[0];
        int group = 0;
        for (int i = 0; i < n; ++i) {
            if (nums[ids[i]] - prev <= limit) {
            } else {
                groups.push_back({});
                ++group;
            }
            groups[group].push_back(ids[i]);

            prev = nums[ids[i]];
        }

        for (int group = 0; group < groups.size(); ++group) {
            vector<int> group_elements;
            transform(groups[group].begin(), groups[group].end(),
                      back_inserter(group_elements),
                      [&](const auto p) { return nums[p]; });

            sort(group_elements.begin(), group_elements.end());
            sort(groups[group].begin(), groups[group].end());

            for (int i = 0; i < groups[group].size(); ++i) {
                nums[groups[group][i]] = group_elements[i];
            }
        }

        return nums;
    }
};