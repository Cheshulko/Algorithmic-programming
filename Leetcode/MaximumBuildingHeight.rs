// https://leetcode.com/problems/maximum-building-height

struct Solution;

impl Solution {
    pub fn max_building(n: i32, mut restrictions: Vec<Vec<i32>>) -> i32 {
        use std::collections::BTreeMap;

        let n = n as usize;

        restrictions.sort_by_key(|r| r[1]);

        let mut real = BTreeMap::new();
        real.insert(0, 0);

        for r in restrictions.iter() {
            let &[id, h] = r.as_slice() else { panic!() };
            let (id, h) = (id as usize - 1, h as usize);

            let (&prev_id, &prev_h) = real.range(..id).next_back().unwrap();
            let d = id - prev_id;
            assert!(prev_h <= h);
            let mut real_h = h.min(prev_h + d);

            if let Some((&next_id, &next_h)) = real.range(id..).next() {
                let d = next_id - id;
                assert!(next_h <= next_h);

                real_h = real_h.min(h.min(next_h + d));
            }

            real.insert(id, real_h);
        }

        if !real.contains_key(&(n - 1)) {
            let last = real.last_entry().unwrap();
            let (&prev_id, &prev_h) = (last.key(), last.get());

            real.insert(n - 1, prev_h + (n - 1 - prev_id));
        }

        let mut ans = 0;
        let mut prev_id = 0;
        let mut prev_h = 0;
        for (id, h) in real.into_iter().skip(1) {
            ans = ans.max(h);

            let d = h.abs_diff(prev_h);
            let dn = id - prev_id - d;
            ans = ans.max(h.max(prev_h) + dn / 2);

            prev_id = id;
            prev_h = h;
        }

        ans as i32
    }
}
