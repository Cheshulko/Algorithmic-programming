// https://leetcode.com/problems/maximum-walls-destroyed-by-robots

#[derive(Debug)]
struct Robot {
    pos: i32,
    dist: i32,
    ind: usize,
}

type Wall = i32;

#[derive(Debug)]
enum Obj {
    Robot(Robot),
    Wall(Wall),
}

impl Obj {
    fn pos(&self) -> i32 {
        match self {
            Obj::Robot(robot) => robot.pos,
            Obj::Wall(pos) => *pos,
        }
    }

    fn is_robot(&self) -> bool {
        match self {
            Obj::Robot(_) => true,
            _ => false,
        }
    }
}

struct Solution;

impl Solution {
    pub fn max_walls(robots: Vec<i32>, distance: Vec<i32>, walls: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let robots_pos = robots.clone().into_iter().collect::<HashSet<_>>();

        let mut same_as_robot = 0;
        let walls = walls
            .into_iter()
            .filter(|wall| {
                if robots_pos.contains(wall) {
                    same_as_robot += 1;
                    false
                } else {
                    true
                }
            })
            .collect::<Vec<_>>();

        let robots = robots
            .into_iter()
            .zip(distance.into_iter())
            .enumerate()
            .collect::<Vec<_>>();

        let mut objs = vec![];
        objs.extend(robots.into_iter().map(|(i, (p, d))| {
            Obj::Robot(Robot {
                pos: p,
                dist: d,
                ind: i,
            })
        }));
        objs.extend(walls.into_iter().map(Obj::Wall));

        objs.push(Obj::Robot(Robot {
            pos: i32::MIN,
            dist: 0,
            ind: 0,
        }));
        objs.push(Obj::Robot(Robot {
            pos: i32::MAX,
            dist: 0,
            ind: 0,
        }));

        objs.sort_unstable_by_key(|a| a.pos());

        let robot_indxs = objs
            .iter()
            .enumerate()
            .filter_map(|(i, r)| r.is_robot().then_some(i))
            .collect::<Vec<_>>();

        fn calc_right_to_left(objs: &[Obj]) -> usize {
            let n = objs.len();
            if n == 2 {
                return 0;
            }

            assert!(objs[0].is_robot());
            let Obj::Robot(right_r) = &objs[n - 1] else {
                panic!()
            };

            let p = objs[1..n - 1].partition_point(|obj| obj.pos() + right_r.dist < right_r.pos);
            n - 1 - (p + 1)
        }

        fn calc_left_to_right(objs: &[Obj]) -> usize {
            let n = objs.len();
            if n == 2 {
                return 0;
            }

            let Obj::Robot(left_r) = &objs[0] else {
                panic!()
            };
            assert!(objs[n - 1].is_robot());

            let p = objs[1..n - 1].partition_point(|obj| obj.pos() <= left_r.pos + left_r.dist);
            p
        }

        let robots_n = robot_indxs.len();

        let mut ans = 0;
        let mut dp_left = vec![0; robots_n];
        let mut dp_right = vec![0; robots_n];

        // 0 ... 1 .. .... .. n - 2 .. n - 1 (robots_n)
        for ro_i in 1..robots_n - 1 {
            let prev_robot_i = robot_indxs[ro_i - 1];
            let cur_robot_i = robot_indxs[ro_i];
            let next_robot_i = robot_indxs[ro_i + 1];

            let left_range = &objs[prev_robot_i..=cur_robot_i];
            let right_range = &objs[cur_robot_i..=next_robot_i];

            let l_r_left_range = calc_left_to_right(left_range);
            let r_l_left_range = calc_right_to_left(left_range);

            dp_left[ro_i] = dp_left[ro_i].max(dp_left[ro_i - 1] + r_l_left_range);
            dp_left[ro_i] = dp_left[ro_i].max(
                (dp_right[ro_i - 1] - l_r_left_range)
                    + (l_r_left_range + r_l_left_range).min(left_range.len() - 2),
            );

            let l_r_right_range = calc_left_to_right(right_range);

            dp_right[ro_i] = dp_right[ro_i].max(dp_left[ro_i - 1] + l_r_right_range);
            dp_right[ro_i] = dp_right[ro_i].max(dp_right[ro_i - 1] + l_r_right_range);

            ans = ans.max(dp_left[ro_i]);
            ans = ans.max(dp_right[ro_i]);
        }

        same_as_robot + ans as i32
    }
}
