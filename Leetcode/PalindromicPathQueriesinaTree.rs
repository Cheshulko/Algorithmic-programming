// https://leetcode.com/problems/palindromic-path-queries-in-a-tree

mod cm_euler_tour {
    pub fn dfs(
        cur: usize,
        adj: &Vec<Vec<usize>>,
        p: i32,
        time: &mut usize,
        tin: &mut Vec<usize>,
        tout: &mut Vec<usize>,
    ) {
        tin[cur] = *time;
        *time += 1;

        for &to in adj[cur].iter() {
            if to as i32 == p {
                continue;
            }
            dfs(to, adj, cur as i32, time, tin, tout);
        }

        tout[cur] = *time;
        *time += 1;
    }
}

mod cm_lca {
    pub struct LCA {
        up: Vec<Vec<usize>>,
        depth: Vec<usize>,
        max_log: usize,
    }

    impl LCA {
        pub fn new(root: usize, adj: &Vec<Vec<usize>>) -> Self {
            let n = adj.len();
            let max_log = (n as f64).log2().ceil() as usize + 1;
            let mut up = vec![vec![root; max_log]; n];
            let mut depth = vec![0; n];

            Self::dfs(root, root, 0, adj, &mut up, &mut depth);

            // Fill binary lifting table
            for i in 1..max_log {
                for v in 0..n {
                    let mid = up[v][i - 1];
                    up[v][i] = up[mid][i - 1];
                }
            }

            LCA { up, depth, max_log }
        }

        fn dfs(
            u: usize,
            p: usize,
            d: usize,
            adj: &Vec<Vec<usize>>,
            up: &mut Vec<Vec<usize>>,
            depth: &mut Vec<usize>,
        ) {
            depth[u] = d;
            up[u][0] = p;
            for &v in &adj[u] {
                if v != p {
                    Self::dfs(v, u, d + 1, adj, up, depth);
                }
            }
        }

        pub fn get_parent(&self, u: usize) -> usize {
            return self.up[u][0];
        }

        pub fn get_lca(&self, mut u: usize, mut v: usize) -> usize {
            if self.depth[u] < self.depth[v] {
                std::mem::swap(&mut u, &mut v);
            }

            // depth[u] >= depth[v]

            // 1. Lift u to the same depth as v
            let diff = self.depth[u] - self.depth[v];
            for i in 0..self.max_log {
                if (diff >> i) & 1 == 1 {
                    u = self.up[u][i];
                }
            }

            if u == v {
                return u;
            }

            // 2. Lift both until they are just below the LCA
            for i in (0..self.max_log).rev() {
                if self.up[u][i] != self.up[v][i] {
                    u = self.up[u][i];
                    v = self.up[v][i];
                }
            }

            self.up[u][0]
        }
    }
}

mod cm_fenwick {
    // [l; r)
    pub struct Fenwick<T> {
        ary: Vec<T>,
    }

    impl<T: Clone + Default + std::ops::AddAssign<T>> Fenwick<T> {
        /// - Time: O(n)
        /// - Space: O(n)
        pub fn new(n: usize) -> Self {
            Fenwick {
                ary: vec![T::default(); n],
            }
        }

        /// - Time: O(n)
        /// - Space: O(n)
        pub fn build_on_array(a: &[T]) -> Self {
            let mut ary = a.to_vec();
            for i in 0..a.len() {
                let j = i | (i + 1);
                if j < a.len() {
                    let tmp = ary[i].clone();
                    ary[j] += tmp;
                }
            }
            Fenwick { ary }
        }

        fn accum(&self, mut idx: usize) -> T {
            let mut sum = T::default();
            while idx > 0 {
                sum += self.ary[idx - 1].clone();
                idx &= idx - 1;
            }
            sum
        }

        // O(log n)
        pub fn add(&mut self, mut idx: usize, val: T) {
            while idx < self.ary.len() {
                self.ary[idx] += val.clone();
                idx |= idx + 1;
            }
        }

        /// [range.start, range.end). O(log n)
        pub fn sum(&self, range: std::ops::Range<usize>) -> T
        where
            T: std::ops::Sub<Output = T>,
        {
            self.accum(range.end) - self.accum(range.start)
        }
    }
}

#[derive(Default, Clone, Debug)]
struct Node {
    chars_cnt: [i32; Node::SZ],
}

impl Node {
    const SZ: usize = 26;
}

impl From<&[char]> for Node {
    fn from(value: &[char]) -> Self {
        let mut node = Node::default();
        for &ch in value {
            let ch = (ch as u8 - b'a') as usize;
            assert!(ch < Node::SZ);
            node.chars_cnt[ch] += 1;
        }

        node
    }
}

impl From<char> for Node {
    fn from(value: char) -> Self {
        let mut node = Node::default();
        let value = (value as u8 - b'a') as usize;
        assert!(value < Node::SZ);
        node.chars_cnt[value] += 1;

        node
    }
}

impl std::ops::Neg for Node {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        for c in self.chars_cnt.iter_mut() {
            *c = -1 * *c;
        }
        self
    }
}

impl std::ops::Add for Node {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for c in 0..Node::SZ {
            self.chars_cnt[c] += rhs.chars_cnt[c];
        }
        self
    }
}

impl std::ops::Sub for Node {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl std::ops::AddAssign for Node {
    fn add_assign(&mut self, rhs: Self) {
        let lhs = std::mem::take(self);
        let result = lhs + rhs;
        *self = result;
    }
}

impl std::ops::SubAssign for Node {
    fn sub_assign(&mut self, rhs: Self) {
        let lhs = std::mem::take(self);
        let result = lhs - rhs;
        *self = result;
    }
}

struct Solution;

impl Solution {
    pub fn palindrome_path(
        n: i32,
        edges: Vec<Vec<i32>>,
        s: String,
        queries: Vec<String>,
    ) -> Vec<bool> {
        let n = n as usize;

        let adj = edges.into_iter().fold(vec![vec![]; n], |mut adj, edge| {
            let (v, u) = (edge[0] as usize, edge[1] as usize);
            adj[v].push(u);
            adj[u].push(v);
            adj
        });

        let mut s = s.chars().collect::<Vec<_>>();

        let mut tin = vec![0; n];
        let mut tout = vec![0; n];
        let mut time = 0;
        cm_euler_tour::dfs(0, &adj, -1, &mut time, &mut tin, &mut tout);

        let lca = cm_lca::LCA::new(0, &adj);

        let mut tree: cm_fenwick::Fenwick<Node> = cm_fenwick::Fenwick::new(2 * n);
        for i in 0..n {
            tree.add(tin[i], Node::from(s[i]));
            tree.add(tout[i], -Node::from(s[i]));
        }

        let mut ans = vec![];
        for query in queries {
            let mut parts = query.split(' ');

            match parts.next() {
                None => unreachable!(),
                Some("update") => {
                    let u = parts.next().unwrap().parse::<usize>().unwrap();
                    let c = parts.next().unwrap().chars().next().unwrap();

                    tree.add(tin[u], Node::from(c) - Node::from(s[u]));
                    tree.add(tout[u], Node::from(s[u]) - Node::from(c));

                    s[u] = c;
                }
                Some("query") => {
                    let u = parts.next().unwrap().parse::<usize>().unwrap();
                    let v = parts.next().unwrap().parse::<usize>().unwrap();

                    let lca_vu = lca.get_lca(u, v);

                    let mut node = tree.sum(0..tin[u] + 1) + tree.sum(0..tin[v] + 1)
                        - tree.sum(0..tin[lca_vu] + 1);
                    let parent_lca_vu = lca.get_parent(lca_vu);
                    if parent_lca_vu != lca_vu {
                        node -= tree.sum(0..tin[parent_lca_vu] + 1);
                    }

                    let cnt = node.chars_cnt.iter().sum::<i32>() as usize;
                    let odd = node.chars_cnt.iter().filter(|&x| x % 2 == 1).count();
                    ans.push(odd <= cnt % 2);
                }
                _ => unreachable!(),
            }
        }

        ans
    }
}
