// https://leetcode.com/problems/stone-game-ii

class Solution {
   public:
    int solve(int I,
              int M,
              int turn,
              const vector<int>& piles,
              vector<vector<array<int, 2>>>& dp) {
        const int N = piles.size();
        const int Size = N - I;
        assert(Size >= 0);
        if (Size == 0) {
            return 0;
        }

        if (dp[I][M][turn] != -1) {
            return dp[I][M][turn];
        }

        int left = 0;
        for (int take = 1; take <= min(Size, 2 * M); ++take) {
            left += piles[I + take - 1];

            int next = solve(I + take, max(M, take), turn ^ 1, piles, dp);
            assert(next >= 0);

            if (turn) {
                dp[I][M][turn] = max(dp[I][M][turn], left + next);
            } else {
                dp[I][M][turn] =
                    dp[I][M][turn] == -1 ? next : min(dp[I][M][turn], next);
            }
        }

        return dp[I][M][turn];
    }

    int stoneGameII(vector<int>& piles) {
        const int n = piles.size();
        const int MAX = 10000;

        vector<vector<array<int, 2>>> dp(
            n + 1, vector<array<int, 2>>(MAX + 1, {-1, -1}));

        return solve(0, 1, 1, piles, dp);
    }
};