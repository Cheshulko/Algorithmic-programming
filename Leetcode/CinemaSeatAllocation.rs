// https://leetcode.com/problems/cinema-seat-allocation

struct Solution;

impl Solution {
    pub fn max_number_of_families(n: i32, mut reserved_seats: Vec<Vec<i32>>) -> i32 {
        reserved_seats.sort_unstable();

        let m = reserved_seats.len();

        let mut ans = 2 * n;
        let mut prev = reserved_seats[0][0];
        let mut i = 0;
        while i < m {
            prev = reserved_seats[i][0];

            let (mut can0, mut can1, mut can2) = (true, true, true);
            while i < m && reserved_seats[i][0] == prev {
                if reserved_seats[i][1] >= 2 && reserved_seats[i][1] <= 5 {
                    can0 = false;
                }
                if reserved_seats[i][1] >= 4 && reserved_seats[i][1] <= 7 {
                    can1 = false;
                }
                if reserved_seats[i][1] >= 6 && reserved_seats[i][1] <= 9 {
                    can2 = false;
                }
                i += 1;
            }

            ans -= 2;
            if can0 && can2 {
                ans += 2;
            } else if can0 || can1 || can2 {
                ans += 1;
            }
        }

        ans
    }
}
