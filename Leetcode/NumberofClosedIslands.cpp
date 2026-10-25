// https://leetcode.com/problems/number-of-closed-islands

class Solution {
   public:
    bool dfs(int i,
             int j,
             const vector<vector<int>>& grid,
             vector<vector<int>>& seen) {
        seen[i][j] = true;

        int n = grid.size(), m = grid[0].size();

        bool is = true;
        for (int di = -1; di <= 1; ++di) {
            for (int dj = -1; dj <= 1; ++dj) {
                if (abs(di + dj) != 1) {
                    continue;
                }
                int to_i = i + di, to_j = j + dj;
                if (to_i < 0 || to_i == n || to_j < 0 || to_j == m) {
                    is = false;
                    continue;
                }
                if (seen[to_i][to_j]) {
                    continue;
                }
                if (grid[to_i][to_j] == 0) {
                    is &= dfs(to_i, to_j, grid, seen);
                }
            }
        }

        return is;
    }

    int closedIsland(vector<vector<int>>& grid) {
        int n = grid.size(), m = grid[0].size();

        int cnt = 0;
        vector<vector<int>> seen(n, vector<int>(m, 0));
        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < m; ++j) {
                if (seen[i][j] || grid[i][j] == 1) {
                    continue;
                }
                if (dfs(i, j, grid, seen)) {
                    ++cnt;
                }
            }
        }

        return cnt;
    }
};