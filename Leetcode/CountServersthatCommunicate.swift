// https://leetcode.com/problems/count-servers-that-communicate

class Solution {
    func countServers(_ grid: [[Int]]) -> Int {
        let (n, m) = (grid.count, grid[0].count)

        var columns = Array(repeating: 0, count: m)
        var rows = Array(repeating: 0, count: n)
        for i in 0..<n {
            for j in 0..<m where grid[i][j] == 1 {
                columns[j] += 1
                rows[i] += 1
            }
        }

        return grid.enumerated().map({ i, row in 
            row.enumerated().filter({ j, cell in 
                (cell == 1) && (columns[j] > 1 || rows[i] > 1)
            }).count
        }).reduce(0, +)
    }
}