// https://leetcode.com/problems/minimum-operations-to-make-a-uni-value-grid

class Solution {
   public:
    int minOperations(vector<vector<int>>& grid, int x) {
        const auto rem0 = grid[0][0] % x;

        const auto n = grid.size();
        const auto m = grid[0].size();

        vector<int> cnt;
        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < m; ++j) {
                if (grid[i][j] % x != rem0) {
                    return -1;
                }
                cnt.push_back(grid[i][j]);
            }
        }

        sort(cnt.begin(), cnt.end());

        int ans = 0;
        for (int i = 0; i < cnt.size(); ++i) {
            ans += abs(cnt[i] - cnt[cnt.size() / 2]) / x;
        }

        return ans;
    }
};