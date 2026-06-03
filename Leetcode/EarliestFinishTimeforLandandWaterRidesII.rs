// https://leetcode.com/problems/earliest-finish-time-for-land-and-water-rides-ii

struct Solution;

impl Solution {
    pub fn earliest_finish_time(
        land_start_time: Vec<i32>,
        land_duration: Vec<i32>,
        water_start_time: Vec<i32>,
        water_duration: Vec<i32>,
    ) -> i32 {
        use std::cmp::Ordering;

        let mut land_start_time = land_start_time
            .into_iter()
            .zip(land_duration.into_iter())
            .collect::<Vec<_>>();

        land_start_time.sort_unstable_by(|&(s1, t1), &(s2, t2)| match (s1 + t1).cmp(&(s2 + t2)) {
            Ordering::Equal => s1.cmp(&s2),
            x => x,
        });

        let mut water_start_time = water_start_time
            .into_iter()
            .zip(water_duration.into_iter())
            .collect::<Vec<_>>();

        water_start_time.sort_unstable_by(|&(s1, t1), &(s2, t2)| match (s1 + t1).cmp(&(s2 + t2)) {
            Ordering::Equal => s1.cmp(&s2),
            x => x,
        });

        let (n, m) = (land_start_time.len(), water_start_time.len());
        let mut ans = i32::MAX;

        let mut mi = i32::MAX;
        let mut j = 0;
        for i in 0..n {
            let end = land_start_time[i].0 + land_start_time[i].1;
            while j < m && water_start_time[j].0 < end {
                mi = mi.min(water_start_time[j].1);
                j += 1;
            }
            if mi != i32::MAX {
                ans = ans.min(end + mi);
            }
            if j < m {
                ans = ans.min(water_start_time[j].0 + water_start_time[j].1);
            } else {
                break;
            }
        }

        let mut mi = i32::MAX;
        let mut i = 0;
        for j in 0..m {
            let end = water_start_time[j].0 + water_start_time[j].1;
            while i < n && land_start_time[i].0 < end {
                mi = mi.min(land_start_time[i].1);
                i += 1;
            }
            if mi != i32::MAX {
                ans = ans.min(end + mi);
            }
            if i < n {
                ans = ans.min(land_start_time[i].0 + land_start_time[i].1);
            } else {
                break;
            }
        }

        ans
    }
}
