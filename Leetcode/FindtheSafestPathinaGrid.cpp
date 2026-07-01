// https://leetcode.com/problems/find-the-safest-path-in-a-grid

class Solution {
   public:
    int maximumSafenessFactor(vector<vector<int>>& grid) {
        const auto n = grid.size(), m = grid[0].size();

        queue<pair<int, int>> q;
        for (size_t i = 0; i < n; ++i) {
            for (size_t j = 0; j < m; ++j) {
                if (grid[i][j]) {
                    q.push({i, j});
                }
            }
        }

        static constexpr array<pair<int, int>, 4> Dirs = {
            {{0, 1}, {0, -1}, {1, 0}, {-1, 0}}};
        while (!q.empty()) {
            const auto [i, j] = q.front();
            const auto cur = grid[i][j];
            q.pop();

            for (const auto [di, dj] : Dirs) {
                const auto toi = i + di, toj = j + dj;
                if (!(toi >= 0 && toi < n && toj >= 0 && toj < m)) {
                    continue;
                }
                if (grid[toi][toj] == 0 || grid[toi][toj] > cur + 1) {
                    grid[toi][toj] = cur + 1;
                    q.push({toi, toj});
                }
            }
        }

        grid[0][0] *= -1;
        priority_queue<pair<int, pair<int, int>>,
                       vector<pair<int, pair<int, int>>>,
                       greater<pair<int, pair<int, int>>>>
            qq;

        qq.push({grid[0][0], {0, 0}});
        while (!qq.empty()) {
            const auto [v, ij] = qq.top();
            const auto [i, j] = ij;
            const auto cur = grid[i][j];
            assert(cur <= 0);
            qq.pop();

            for (const auto [di, dj] : Dirs) {
                const auto toi = i + di, toj = j + dj;
                if (!(toi >= 0 && toi < n && toj >= 0 && toj < m)) {
                    continue;
                }
                if (grid[toi][toj] >= 0) {
                    grid[toi][toj] = -min(-cur, grid[toi][toj]);
                    qq.push({grid[toi][toj], {toi, toj}});
                }
            }
        }

        return -grid[n - 1][m - 1] - 1;
    }
};