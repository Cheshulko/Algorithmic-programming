// https://leetcode.com/problems/count-zero-request-servers/

struct Solution;

impl Solution {
    pub fn count_servers(n: i32, mut logs: Vec<Vec<i32>>, x: i32, queries: Vec<i32>) -> Vec<i32> {
        logs.sort_unstable_by_key(|x| x[1]);

        let mut queries = queries.into_iter().enumerate().collect::<Vec<_>>();
        queries.sort_unstable_by_key(|x| x.1);

        let n = n as usize;
        let mut received = vec![0; n];
        let mut cnt = 0;

        let mut left = 0;
        let mut right = 0;

        let mut ans = vec![0; queries.len()];
        for (i, query) in queries {
            while right < logs.len() && logs[right][1] <= query {
                let server = logs[right][0] as usize - 1;
                received[server] += 1;
                if received[server] == 1 {
                    cnt += 1;
                }
                right += 1;
            }

            while left < logs.len() && logs[left][1] < query - x {
                let server = logs[left][0] as usize - 1;
                received[server] -= 1;
                if received[server] == 0 {
                    cnt -= 1;
                }
                left += 1;
            }

            ans[i] = n as i32 - cnt;
        }

        ans
    }
}
