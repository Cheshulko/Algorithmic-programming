// https://leetcode.com/problems/valid-parenthesis-string

class Solution {
   public:
    bool checkValidString(string s) {
        deque<int> have;

        int sum = 0;
        for (int i = 0; i < s.size(); ++i) {
            char c = s[i];
            if (c == '*') {
                have.push_back(i);
            }

            sum += c == '(';
            sum -= c == ')';

            if (sum < 0) {
                if (!have.empty()) {
                    int j = have.front();
                    have.pop_front();
                    s[j] = '(';

                    ++sum;
                } else {
                    return false;
                }
            }
        }

        while (sum > 0 && !have.empty()) {
            int j = have.back();
            have.pop_back();
            s[j] = ')';
            --sum;
        }

        sum = 0;
        for (char c : s) {
            sum += c == '(';
            sum -= c == ')';

            if (sum < 0) {
                return false;
            }
        }

        return sum == 0;
    }
};