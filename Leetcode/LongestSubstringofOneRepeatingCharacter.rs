use std::collections::{BTreeSet, BinaryHeap, HashMap};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Segment {
    start: usize,
    end: usize,
    id: usize,
}

impl Segment {
    pub fn fake() -> Self {
        Segment {
            start: usize::MAX,
            end: usize::MIN,
            id: usize::MAX,
        }
    }
}

struct Segmnents {
    valid_ids: HashMap<usize, char>,
    segments: BTreeSet<Segment>,
    max_ord: BinaryHeap<(usize, usize)>,
    segment_id: usize,
}

impl Segmnents {
    pub fn new() -> Self {
        Self {
            valid_ids: HashMap::new(),
            segments: BTreeSet::new(),
            max_ord: BinaryHeap::new(),
            segment_id: 0,
        }
    }

    fn new_seg(&mut self, start: usize, end: usize) -> Segment {
        let id = self.segment_id;
        self.segment_id += 1;

        Segment { start, end, id }
    }

    pub fn remove(&mut self, segment: &Segment) {
        self.segments.remove(&segment);
        self.valid_ids.remove(&segment.id);
    }

    pub fn insert(&mut self, start: usize, end: usize, ch: char) {
        let segment = self.new_seg(start, end);
        let cnt = segment.end - segment.start + 1;

        self.max_ord.push((cnt, segment.id));
        self.valid_ids.insert(segment.id, ch);
        self.segments.insert(segment);
    }

    pub fn find(&self, i: usize) -> Option<(Segment, char)> {
        let segment = self
            .segments
            .range(
                ..Segment {
                    start: i,
                    end: usize::MAX,
                    id: usize::MAX,
                },
            )
            .next_back();

        return segment.map(|s| (*s, *self.valid_ids.get(&s.id).unwrap()));
    }

    pub fn prev(&self, segment: &Segment) -> Option<(Segment, char)> {
        let segment = self
            .segments
            .range(
                ..Segment {
                    start: segment.start,
                    end: usize::MAX,
                    id: usize::MAX,
                },
            )
            .next_back();

        return segment.map(|s| (*s, *self.valid_ids.get(&s.id).unwrap()));
    }

    pub fn next(&self, segment: &Segment) -> Option<(Segment, char)> {
        let segment = self
            .segments
            .range(
                Segment {
                    start: segment.end,
                    end: usize::MIN,
                    id: usize::MIN,
                }..,
            )
            .next();

        return segment.map(|s| (*s, *self.valid_ids.get(&s.id).unwrap()));
    }

    pub fn max(&mut self) -> i32 {
        while let Some(&top) = self.max_ord.peek() {
            if self.valid_ids.contains_key(&top.1) {
                return top.0 as i32;
            } else {
                self.max_ord.pop();
            }
        }
        unreachable!()
    }
}

impl Solution {
    pub fn longest_repeating(
        s: String,
        query_characters: String,
        query_indices: Vec<i32>,
    ) -> Vec<i32> {
        let n = s.len();
        let s = s.chars().collect::<Vec<_>>();

        let mut segments = Segmnents::new();

        let mut prev = s[0];
        let mut start = 0;
        for (i, &c) in s.iter().enumerate().skip(1) {
            if c != prev {
                segments.insert(start, i - 1, prev);

                prev = c;
                start = i;
            }
        }
        segments.insert(start, n - 1, prev);

        let mut ans = vec![];
        for (c, i) in query_characters.chars().zip(query_indices.into_iter()) {
            let i = i as usize;

            let (segment, segment_ch) = segments.find(i).unwrap();

            if segment_ch == c {
                // nothing
                ans.push(segments.max());
                continue;
            }

            segments.remove(&segment);

            let prev_segment = segments.prev(&segment);
            let next_segment = segments.next(&segment);

            match (
                segment.start == i,
                segment.end == i,
                prev_segment,
                next_segment,
            ) {
                (true, true, maybe_prev_segment, maybe_next_segment) => {
                    // prev_seg I next_segment
                    let (prev_segment, prev_segment_ch) =
                        maybe_prev_segment.unwrap_or((Segment::fake(), '#'));
                    let (next_segment, next_segment_ch) =
                        maybe_next_segment.unwrap_or((Segment::fake(), '#'));

                    match (prev_segment_ch == c, next_segment_ch == c) {
                        (true, true) => {
                            segments.remove(&prev_segment);
                            segments.remove(&next_segment);
                            segments.insert(prev_segment.start, next_segment.end, c);
                        }
                        (true, false) => {
                            segments.remove(&prev_segment);
                            segments.insert(prev_segment.start, i, c);
                        }
                        (false, true) => {
                            segments.remove(&next_segment);
                            segments.insert(i, next_segment.end, c);
                        }
                        (false, false) => {
                            segments.insert(i, i, c);
                        }
                    }
                }
                (true, false, maybe_prev_segment, _) => {
                    // prev_seg I
                    let (prev_segment, prev_segment_ch) =
                        maybe_prev_segment.unwrap_or((Segment::fake(), '#'));

                    if prev_segment_ch == c {
                        segments.remove(&prev_segment);
                        segments.insert(prev_segment.start, i, c);
                        segments.insert(i + 1, segment.end, segment_ch);
                    } else {
                        segments.insert(i, i, c);
                        segments.insert(i + 1, segment.end, segment_ch);
                    }
                }
                (false, true, _, maybe_next_segment) => {
                    // I next_segment
                    let (next_segment, next_segment_ch) =
                        maybe_next_segment.unwrap_or((Segment::fake(), '#'));

                    if next_segment_ch == c {
                        segments.remove(&next_segment);
                        segments.insert(i, next_segment.end, c);
                        segments.insert(segment.start, i - 1, segment_ch);
                    } else {
                        segments.insert(segment.start, i - 1, segment_ch);
                        segments.insert(i, i, c);
                    }
                }
                _ => {
                    segments.insert(segment.start, i - 1, segment_ch);
                    segments.insert(i, i, c);
                    segments.insert(i + 1, segment.end, segment_ch);
                }
            }

            ans.push(segments.max());
        }

        ans
    }
}
