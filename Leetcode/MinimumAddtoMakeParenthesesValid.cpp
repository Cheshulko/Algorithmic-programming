// https://leetcode.com/problems/minimum-add-to-make-parentheses-valid

class Solution {
   public:
    int minAddToMakeValid(string s) {
        int sum = 0;
        int ans = 0;
        for (auto c : s) {
            sum += c == '(';
            sum -= c == ')';

            if (sum == -1) {
                sum = 0;
                ++ans;
            }
        }

        return ans + sum;
    }
};