// https://leetcode.com/problems/find-the-length-of-the-longest-common-prefix

class Solution {
   public:
    int longestCommonPrefix(vector<int>& arr1, vector<int>& arr2) {
        const auto get = [](int num) -> vector<int> {
            vector<int> digits;
            while (num) {
                digits.push_back(num % 10);
                num /= 10;
            }

            reverse(digits.begin(), digits.end());

            return digits;
        };

        set<int> nums;
        for (auto num : arr2) {
            int x = 0;
            for (const auto d : get(num)) {
                x = x * 10 + d;
                nums.insert(x);
            }
        }

        int ans = 0;
        for (auto num : arr1) {
            int x = 0;
            int len = 0;
            for (const auto d : get(num)) {
                x = x * 10 + d;
                len += 1;
                if (nums.cend() != nums.find(x)) {
                    ans = max(ans, len);
                }
            }
        }

        return ans;
    }
};