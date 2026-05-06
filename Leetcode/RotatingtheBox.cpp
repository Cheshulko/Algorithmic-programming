// https://leetcode.com/problems/rotating-the-box

class Solution {
   public:
    vector<vector<char>> rotateTheBox(vector<vector<char>>& boxGrid) {
        const auto n = boxGrid.size();
        const auto m = boxGrid[0].size();

        vector<vector<char>> ans(m, vector<char>(n, '.'));
        for (int j = 0; j < m; ++j) {
            for (int i = 0; i < n; ++i) {
                ans[j][i] = boxGrid[n - 1 - i][j];
            }
        }

        for (int j = 0; j < n; ++j) {
            for (int k = 0; k < m; ++k) {
                for (int i = 0; i < m - 1; ++i) {
                    if (ans[i + 1][j] == '.' && ans[i][j] == '#') {
                        swap(ans[i + 1][j], ans[i][j]);
                    }
                }
            }
        }

        return ans;
    }
};