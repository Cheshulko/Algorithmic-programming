// https://leetcode.com/problems/count-the-number-of-complete-components

class Solution {
   public:
    int countCompleteComponents(int n, vector<vector<int>>& edges) {
        auto dfs = [](auto self, int cur, const vector<vector<int>>& adj,
                      vector<bool>& seen, vector<int>& comp) -> void {
            seen[cur] = true;
            comp.push_back(cur);

            for (const auto to : adj[cur]) {
                if (!seen[to]) {
                    self(self, to, adj, seen, comp);
                }
            }
        };

        vector<vector<int>> adj(n, vector<int>());
        for (auto edge : edges) {
            adj[edge[0]].push_back(edge[1]);
            adj[edge[1]].push_back(edge[0]);
        }

        int ans = 0;
        vector<bool> seen(n, false);
        for (auto i = 0; i < n; ++i) {
            if (!seen[i]) {
                vector<int> comp;
                dfs(dfs, i, adj, seen, comp);

                const auto m = comp.size();
                bool ok = true;
                for (const auto j : comp) {
                    if (adj[j].size() != m - 1) {
                        ok = false;
                        break;
                    }
                }

                ans += ok;
            }
        }

        return ans;
    }
};