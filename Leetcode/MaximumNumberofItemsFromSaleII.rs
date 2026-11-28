// https://leetcode.com/problems/maximum-number-of-items-from-sale-ii

struct Solution;

impl Solution {
    pub fn maximum_sale_items(items: Vec<Vec<i32>>, mut budget: i32) -> i32 {
        let n = items.len();
        let ma_factor = items.iter().map(|it| it[0]).max().unwrap() as usize;

        let mut cnt = vec![0; ma_factor + 1];
        for item in items.iter() {
            cnt[item[0] as usize] += 1;
        }
        for factor in 1..=ma_factor {
            for cur_factor in (2 * factor..=ma_factor).step_by(factor) {
                cnt[factor] += cnt[cur_factor];
            }
        }

        let mut items = items
            .into_iter()
            .map(|it| {
                let factor = it[0] as usize;
                let price = it[1];

                (price, cnt[factor] - 1)
            })
            .collect::<Vec<_>>();

        items.sort_unstable();

        let mi_price = items[0].0;

        let mut ans = 0;
        let mut i = 0;
        while budget >= mi_price {
            if i == n || items[i].0 >= 2 * mi_price {
                let take = budget / mi_price;
                ans += take;

                break;
            }

            if items[i].1 > 0 {
                let can_take = budget / items[i].0;
                let take = can_take.min(items[i].1);

                ans += 2 * take;
                budget -= items[i].0 * take;
            }
            i += 1;
        }

        ans
    }
}
