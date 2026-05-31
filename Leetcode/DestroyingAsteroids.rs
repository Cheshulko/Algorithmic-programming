// https://leetcode.com/problems/destroying-asteroids

struct Solution;

impl Solution {
    pub fn asteroids_destroyed(mass: i32, mut asteroids: Vec<i32>) -> bool {
        asteroids.sort_unstable();

        let mut mass = mass as i64;
        for ast in asteroids {
            let ast = ast as i64;
            if ast > mass {
                return false;
            }
            mass += ast;
        }

        true
    }
}
