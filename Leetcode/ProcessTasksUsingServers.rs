impl Solution {
    pub fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut free = servers
            .into_iter()
            .enumerate()
            .map(|(i, s)| Reverse((s, i)))
            .collect::<BinaryHeap<_>>();

        let mut processing: BinaryHeap<Reverse<(usize, i32, usize)>> = BinaryHeap::new();

        tasks
            .into_iter()
            .enumerate()
            .scan(0, |tt, (ti, t)| {
                *tt = (*tt).max(ti);

                if free.is_empty() {
                    *tt = (*tt).max(processing.peek().unwrap().0 .0);
                }

                while let Some(Reverse((time, s, i))) = processing.peek().cloned() {
                    if time <= *tt {
                        processing.pop();
                        free.push(Reverse((s, i)));
                    } else {
                        break;
                    }
                }

                let Some(Reverse((s, i))) = free.pop() else {
                    panic!()
                };

                processing.push(Reverse(((*tt) + t as usize, s, i)));

                Some(i as i32)
            })
            .collect()
    }
}
