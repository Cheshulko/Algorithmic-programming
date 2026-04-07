// https://leetcode.com/problems/walking-robot-simulation-ii

const DIRS: &[(i32, i32)] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];

struct Robot {
    r_i: usize,
    r_j: usize,
    r_d: usize,
    width: usize,
    height: usize,
}

impl Robot {
    fn new(width: i32, height: i32) -> Self {
        Self {
            r_i: 0,
            r_j: 0,
            r_d: 1,
            width: width as usize,
            height: height as usize,
        }
    }

    fn step(&mut self, mut num: i32) {
        let cycle = 2 * (self.height + self.width - 2);
        num %= cycle as i32;
        if num == 0 {
            num = cycle as i32;
        }
        while num > 0 {
            let (di, dj) = DIRS[self.r_d];
            let to_i = self.r_i as i32 + di;
            let to_j = self.r_j as i32 + dj;

            if to_i < 0 || to_i == self.width as i32 || to_j < 0 || to_j == self.height as i32 {
                self.r_d = (self.r_d + 4 - 1) % 4;
                continue;
            }

            self.r_i = to_i as usize;
            self.r_j = to_j as usize;
            num -= 1;
        }
    }

    fn get_pos(&self) -> Vec<i32> {
        vec![self.r_i as i32, self.r_j as i32]
    }

    fn get_dir(&self) -> String {
        ["North", "East", "South", "West"][self.r_d].to_string()
    }
}
