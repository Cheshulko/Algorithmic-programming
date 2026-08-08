impl Solution {
    pub fn valid_sequence(word1: String, word2: String) -> Vec<i32> {
        let mut w1 = word1.chars().collect::<Vec<_>>();
        w1.reverse();
        let n = w1.len();

        let mut w2 = word2.chars().collect::<Vec<_>>();
        w2.reverse();
        let m = w2.len();

        let mut dp = vec![0; m + 1];
        for j in 1..=m {
            if dp[j - 1] == n + 1 {
                dp[j] = dp[j - 1];
                continue;
            }

            let mut i = dp[j - 1] + 1;
            while i <= n {
                if w1[i - 1] == w2[j - 1] {
                    dp[j] = i;
                    break;
                }
                i += 1;
            }

            if i == n + 1 {
                dp[j] = i;
            }
        }

        w1.reverse();
        w2.reverse();

        let mut have_hack = true;
        let mut j = 0;
        let mut ans = vec![];
        for i in 0..n {
            if j == m {
                break;
            }
            if w1[i] == w2[j] {
                ans.push(i as i32);
                j += 1;
            } else {
                if have_hack {
                    if (dp[m - j - 1] != n + 1) && n - dp[m - j - 1] > i {
                        have_hack = false;
                        ans.push(i as i32);
                        j += 1
                    } else {
                        continue;
                    }
                }
            }
        }

        if ans.len() != m {
            vec![]
        } else {
            ans
        }
    }
}
