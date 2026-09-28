// https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses

class Solution {
   public:
    int maxDepth(string s) {
        int d = 0, ma = 0;
        for (auto c : s) {
            if (c == '(') {
                ++d;
            } else if (c == ')') {
                --d;
            }
            ma = max(ma, d);
        }

        return ma;
    }
};