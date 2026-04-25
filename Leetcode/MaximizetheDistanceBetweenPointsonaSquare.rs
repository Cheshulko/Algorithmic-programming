// https://leetcode.com/problems/maximize-the-distance-between-points-on-a-square

struct Solution;

impl Solution {
    pub fn max_distance2(side: i32, points: Vec<Vec<i32>>, k: i32) -> i32 {
        let side = side as i64;
        let points = points
            .into_iter()
            .map(|p| (p[0] as i64, p[1] as i64))
            .collect::<Vec<_>>();

        let mut left = points.iter().filter(|&&(x, _)| x == 0).collect::<Vec<_>>();
        let mut right = points
            .iter()
            .filter(|&&(x, _)| x == side)
            .collect::<Vec<_>>();

        let mut up = points
            .iter()
            .filter(|&&(x, y)| y == side && x != 0 && x != side)
            .collect::<Vec<_>>();
        let mut down = points
            .iter()
            .filter(|&&(x, y)| y == 0 && x != 0 && x != side)
            .collect::<Vec<_>>();

        assert!(left.len() + right.len() + up.len() + down.len() == points.len());

        left.sort_unstable();
        right.sort_unstable();
        up.sort_unstable();
        down.sort_unstable();

        let points_sorted = left
            .into_iter()
            .chain(up.into_iter())
            .chain(right.into_iter().rev())
            .chain(down.into_iter().rev())
            .collect::<Vec<_>>();

        fn dist(p1: &(i64, i64), p2: &(i64, i64)) -> i64 {
            (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
        }

        fn can(points_sorted: &[&(i64, i64)], k: i32, d: i64) -> bool {
            let n = points_sorted.len();

            let start_g = points_sorted[0];

            for si in 0..500.min(points_sorted.len()) {
                if dist(start_g, points_sorted[si]) > d {
                    break;
                }

                let mut k = k - 1;

                let start_p = points_sorted[si];
                let mut last_p = start_p;

                for delta in 1..n {
                    let pi = (si + delta) % n;

                    if dist(last_p, points_sorted[pi]) < d {
                        continue;
                    }
                    if points_sorted[pi] != start_p && dist(start_p, points_sorted[pi]) < d {
                        continue;
                    }

                    last_p = points_sorted[pi];
                    k -= 1;
                }

                if k <= 0 {
                    return true;
                }
            }

            false
        }

        let mut l = -1;
        let mut r = 4 * side + 1;
        while r - l > 1 {
            let m = (l + r) >> 1;
            if can(&points_sorted, k, m) {
                l = m;
            } else {
                r = m;
            }
        }

        l as i32
    }
}
