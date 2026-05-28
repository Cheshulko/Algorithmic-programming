// https://leetcode.com/problems/longest-common-suffix-queries

mod cm_trie {
    use std::collections::BTreeMap;

    #[derive(Clone)]
    struct Node {
        next: Vec<Option<usize>>,
        word_lens: BTreeMap<usize, usize>,
    }

    impl Node {
        pub fn new(alphabet_size: usize) -> Self {
            Node {
                next: vec![None; alphabet_size],
                word_lens: BTreeMap::new(),
            }
        }
    }

    pub struct Trie {
        alphabet_size: usize,
        t: Vec<Node>,
    }

    impl Trie {
        pub fn new(alphabet_size: usize) -> Self {
            Trie {
                alphabet_size,
                t: vec![Node::new(alphabet_size); alphabet_size],
            }
        }
        /// Insert a string into the Trie
        /// - Time: O(|s|)
        pub fn insert(&mut self, s: &[u8], index: usize) {
            let mut v = 0;
            _ = self.t[v].word_lens.entry(s.len()).or_insert(index);

            for &ch in s {
                let idx = ch as usize;
                if self.t[v].next[idx].is_none() {
                    self.t[v].next[idx] = Some(self.t.len());
                    self.t.push(Node::new(self.alphabet_size));
                }
                v = self.t[v].next[idx].unwrap();
                _ = self.t[v].word_lens.entry(s.len()).or_insert(index);
            }
        }

        /// Find the prefix for the given input
        /// - Time: O(|s|)
        pub fn longest_prefix(&self, s: &[u8]) -> Option<usize> {
            let get_index = |node: &Node| node.word_lens.values().next().copied();

            let mut v = 0;
            for &ch in s {
                let idx = ch as usize;
                if self.t[v].next[idx].is_none() {
                    return get_index(&self.t[v]);
                }
                v = self.t[v].next[idx].unwrap();
            }
            get_index(&self.t[v])
        }
    }
}

struct Solution;

impl Solution {
    pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
        let mut trie = cm_trie::Trie::new(26);

        let transform =
            |word: String| -> Vec<u8> { word.bytes().rev().map(|b| b - b'a').collect() };

        for (i, word) in words_container.into_iter().enumerate() {
            let bytes = transform(word);
            trie.insert(&bytes, i);
        }

        words_query
            .into_iter()
            .map(|word| {
                let bytes = transform(word);
                trie.longest_prefix(&bytes).unwrap() as i32
            })
            .collect()
    }
}
