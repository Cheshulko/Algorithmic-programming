// https://leetcode.com/problems/path-existence-queries-in-a-graph-ii

class Solution {
    vector<int> pathExistenceQueries(int n,
                                     vector<int>& nums,
                                     int maxDiff,
                                     vector<vector<int>>& queries) {
        vector<pair<int, size_t>> nums_ord;
        nums_ord.reserve(n);
        for (auto i = 0; i < n; ++i) {
            nums_ord.push_back({nums[i], i});
        }
        sort(nums_ord.begin(), nums_ord.end());

        vector<int> to_ind;
        to_ind.reserve(n);
        for (auto i = 0; i < n; ++i) {
            to_ind[nums_ord[i].second] = i;
        }

        const auto P = 32;
        vector<vector<int>> parent(P, vector<int>(n, -1));

        for (auto i = 0, prev = 0; i < n;) {
            i = max(i, prev);

            if (nums_ord[i].first - nums_ord[prev].first <= maxDiff) {
                parent[0][i] = prev;
                ++i;
            } else {
                ++prev;
            }
        }

        for (auto pow = 1; pow < P; ++pow) {
            for (auto i = 0; i < n; ++i) {
                assert(parent[pow - 1][i] >= 0);
                parent[pow][i] = parent[pow - 1][parent[pow - 1][i]];
            }
        }

        const auto m = queries.size();
        vector<int> ans(m, -1);
        for (auto i = 0; i < m; ++i) {
            const auto v = queries[i][0], u = queries[i][1];

            auto id1 = to_ind[v], id2 = to_ind[u];
            if (id1 == id2) {
                ans[i] = 0;
                continue;
            }

            if (id2 > id1) {
                swap(id1, id2);
            }

            auto cnt = 0;
            for (int pow = P - 1; pow >= 0; --pow) {
                if (id1 != parent[pow][id1] && parent[pow][id1] > id2) {
                    id1 = parent[pow][id1];
                    cnt += 1 << pow;
                }
            }

            assert(id1 > id2);
            if (parent[0][id1] <= id2) {
                ans[i] = cnt + 1;
            }
        }

        return ans;
    }
};