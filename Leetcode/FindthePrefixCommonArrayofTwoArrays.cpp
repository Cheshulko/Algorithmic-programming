// https://leetcode.com/problems/find-the-prefix-common-array-of-two-arrays

class Solution {
   public:
    vector<int> findThePrefixCommonArray(vector<int>& A, vector<int>& B) {
        const auto MAX = 50 + 1;
        const auto n = A.size();

        vector<int> seen_a(MAX, false);
        vector<int> seen_b(MAX, false);

        vector<int> ans(n, 0);
        for (int i = 0; i < n; ++i) {
            seen_a[A[i]] = seen_b[B[i]] = true;

            ans[i] += seen_a[B[i]] + seen_b[A[i]] - (A[i] == B[i]);
            if (i > 0) {
                ans[i] += ans[i - 1];
            }
        }

        return ans;
    }
};