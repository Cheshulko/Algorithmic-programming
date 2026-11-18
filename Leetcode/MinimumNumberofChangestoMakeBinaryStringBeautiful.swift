// https://leetcode.com/problems/minimum-number-of-changes-to-make-binary-string-beautiful

class Solution {
    func minChanges(_ s: String) -> Int {
        let s = Array.init(s)

        var cnt = 0
        for i in 1..<s.count {
            if (i & 1) == 1 && s[i - 1] != s[i] {
                cnt += 1
            }
        }

        return cnt
    }
}