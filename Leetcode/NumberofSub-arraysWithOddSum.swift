// https://leetcode.com/problems/number-of-sub-arrays-with-odd-sum

class Solution {
    func numOfSubarrays(_ arr: [Int]) -> Int {
        let M = 1_000_000_000 + 7
        let n = arr.count
        
        var cnt = [0, 0]
        var ans = 0
        for i in 0..<n {
            var ncnt = [0, 0]
            if (arr[i] & 1) == 1 {
                ncnt[0] = cnt[1]
                ncnt[1] = cnt[0] + 1
            } else {
                ncnt[0] = cnt[0] + 1
                ncnt[1] = cnt[1]
            }
            
            cnt = ncnt
            ans = (ans + cnt[1]) % M
        }

        return ans
    }
}