// https://leetcode.com/problems/number-of-substrings-containing-all-three-characters

class Solution {
   public:
    int numberOfSubstrings(string s) {
        const auto n = s.length();
        int ans = 0;
        array<int, 3> seen;

        size_t j = 0;
        for (size_t i = 0; i < n; ++i) {
            j = max(i, j);
            for (; j < n && !(seen[0] && seen[1] && seen[2]); ++j) {
                if (s[j] <= 'c') {
                    seen[s[j] - 'a'] += 1;
                }
            }
            if (seen[0] && seen[1] && seen[2]) {
                ans += n - j + 1;
            }
            if (s[i] <= 'c') {
                seen[s[i] - 'a'] -= 1;
            }
        }

        return ans;
    }
};