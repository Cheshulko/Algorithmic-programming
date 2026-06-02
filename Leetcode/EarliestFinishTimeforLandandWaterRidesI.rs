// https://leetcode.com/problems/earliest-finish-time-for-land-and-water-rides-i

struct Solution;

impl Solution {
    pub fn earliest_finish_time(
        land_start_time: Vec<i32>,
        land_duration: Vec<i32>,
        water_start_time: Vec<i32>,
        water_duration: Vec<i32>,
    ) -> i32 {
        let mut ans = i32::MAX;

        let (n, m) = (land_start_time.len(), water_start_time.len());

        for i in 0..n {
            for j in 0..m {
                let t1 = land_start_time[i] + land_duration[i];
                ans = ans.min(t1 + (water_start_time[j] - t1).max(0) + water_duration[j]);

                let t1 = water_start_time[j] + water_duration[j];
                ans = ans.min(t1 + (land_start_time[i] - t1).max(0) + land_duration[i]);
            }
        }

        ans
    }
}
