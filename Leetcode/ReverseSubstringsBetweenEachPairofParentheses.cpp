// https://leetcode.com/problems/reverse-substrings-between-each-pair-of-parentheses

class Solution {
   public:
    string reverseParentheses(string s) {
        auto n = s.length();

        string ans;
        for (auto c : s) {
            if (c == '(') {
                ans.push_back(c);
            } else if (c == ')') {
                string tmp;
                while (ans.back() != '(') {
                    tmp += ans.back();
                    ans.pop_back();
                }
                ans.pop_back();
                ans += tmp;
            } else {
                ans.push_back(c);
            }
        }

        return ans;
    }
};