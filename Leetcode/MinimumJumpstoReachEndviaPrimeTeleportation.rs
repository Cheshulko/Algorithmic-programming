// https://leetcode.com/problems/minimum-jumps-to-reach-end-via-prime-teleportation

fn eratosthenes(n: usize) -> Vec<bool> {
    let mut pr = vec![true; n + 1];
    pr[0] = false;
    pr[1] = false;
    for i in 2..=n {
        if pr[i] {
            for j in (2 * i..=n).step_by(i) {
                pr[j] = false;
            }
        }
    }

    pr
}

fn unique_trial_division<'a>(
    mut n: usize,
    primes: impl IntoIterator<Item = &'a usize>,
) -> Vec<usize> {
    let mut factorization = vec![];

    for &prime in primes {
        if prime * prime > n {
            break;
        }

        if n % prime == 0 {
            factorization.push(prime);
            while n % prime == 0 {
                n /= prime;
            }
        }
    }

    if n > 1 {
        factorization.push(n);
    }

    factorization
}

struct Solution;

impl Solution {
    pub fn min_jumps(nums: Vec<i32>) -> i32 {
        use std::collections::{HashMap, HashSet, VecDeque};

        let nums = nums.into_iter().map(|n| n as usize).collect::<Vec<_>>();
        let ma = nums.iter().max().copied().unwrap() + 1;
        let n = nums.len();

        let primes = eratosthenes(ma);

        let mut prime_indxs = HashMap::<usize, Vec<usize>>::new();
        for (i, &num) in nums.iter().enumerate() {
            if primes[num] {
                prime_indxs.entry(num).or_default().push(i);
            }
        }

        let primes_list = primes
            .iter()
            .enumerate()
            .filter_map(|(i, pr)| pr.then_some(i))
            .collect::<Vec<_>>();

        let mut adj = vec![vec![]; n];
        for (i, &num) in nums.iter().enumerate() {
            if i > 0 {
                adj[i].push(i - 1);
            }
            if i < n - 1 {
                adj[i].push(i + 1);
            }
            if primes[num] {
                continue;
            }

            let factors = unique_trial_division(num, &primes_list);
            for factor in factors.into_iter() {
                for &ind in prime_indxs.get(&factor).unwrap_or(&vec![]) {
                    adj[ind].push(i);
                }
            }
        }

        let mut queue = VecDeque::new();
        let mut seen = vec![false; n];

        queue.push_back((0, 0));
        seen[0] = true;

        while let Some((ind, dist)) = queue.pop_front() {
            if ind == n - 1 {
                return dist;
            }

            if primes[nums[ind]] {
                for &to_ind in prime_indxs.get(&nums[ind]).unwrap_or(&vec![]) {
                    if !seen[to_ind] {
                        seen[to_ind] = true;
                        queue.push_back((to_ind, dist + 1));
                    }
                }

                prime_indxs.insert(nums[ind], vec![]);
            }

            for &to in adj[ind].iter() {
                if !seen[to] {
                    seen[to] = true;
                    queue.push_back((to, dist + 1));
                }
            }
        }

        -1
    }
}
