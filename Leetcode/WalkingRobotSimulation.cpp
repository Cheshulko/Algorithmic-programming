// https://leetcode.com/problems/walking-robot-simulation/?envType=daily-question&envId=2026-04-06

class Solution {
   public:
    int robotSim(vector<int>& commands, vector<vector<int>>& obstacles) {
        const auto N = 4;
        const auto DIR_Y = array<int, N>{0, 1, 0, -1};
        const auto DIR_X = array<int, N>{-1, 0, 1, 0};

        int dir = 1, x = 0, y = 0;
        int ans = 0;

        set<pair<int, int>> obstacles_m;
        for (const auto& obstacle : obstacles) {
            obstacles_m.insert({obstacle[0], obstacle[1]});
        }

        for (auto command : commands) {
            switch (command) {
                case -2:
                    dir = (dir + N - 1) % N;
                    break;
                case -1:
                    dir = (dir + 1) % N;
                    break;
                default:
                    while (command-- > 0) {
                        auto x_to = x + DIR_X[dir], y_to = y + DIR_Y[dir];
                        if (obstacles_m.cend() !=
                            obstacles_m.find({x_to, y_to})) {
                            break;
                        }
                        x = x_to, y = y_to;
                        ans = max(ans, x * x + y * y);
                    }
            }
        }

        return ans;
    }
};