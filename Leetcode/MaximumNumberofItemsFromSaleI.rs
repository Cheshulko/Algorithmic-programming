impl Solution {
    pub fn maximum_sale_items(items: Vec<Vec<i32>>, budget: i32) -> i32 {
        let n = items.len();

        let mut copies = vec![0; n];
        for i in 0..n {
            let mut cnt = 0;
            for j in 0..n {
                if i == j {
                    continue;
                }
                if items[j][0] % items[i][0] == 0 {
                    cnt += 1;
                }
            }
            copies[i] = cnt + 1;
        }

        let budget = budget as usize;

        let mut dp = vec![vec![0; budget + 1]; n + 1];
        for i in 0..n {
            let price = items[i][1] as usize;
            let cnt = copies[i];

            for b in 0..=budget {
                // not take
                dp[i + 1][b] = dp[i + 1][b].max(dp[i][b]);

                if b >= price && cnt > 1 {
                    dp[i + 1][b] = dp[i + 1][b].max(dp[i][b - price] + cnt);
                }
            }
        }

        let mi = items.iter().map(|it| it[1] as usize).min().unwrap();

        let mut ans = 0;
        for b in 0..=budget {
            ans = ans.max(dp[n][b] + (budget - b) / mi);
        }

        ans as i32
    }
}
