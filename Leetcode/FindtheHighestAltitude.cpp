// https://leetcode.com/problems/find-the-highest-altitude

class Solution {
   public:
    int largestAltitude(vector<int>& gain) {
        int ma = 0, cur = 0;
        for (auto g : gain) {
            cur += g;
            ma = max(cur, ma);
        }

        return ma;
    }
};