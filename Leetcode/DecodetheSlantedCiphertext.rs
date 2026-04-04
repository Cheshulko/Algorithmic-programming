// https://leetcode.com/problems/decode-the-slanted-ciphertext

struct Solution;

impl Solution {
    pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
        let encoded_text = encoded_text.into_bytes();
        let rows = rows as usize;
        assert!(encoded_text.len() % rows == 0);
        let cols = encoded_text.len() / rows;

        if cols == 0 {
            return String::new();
        }

        let mut grid = vec![vec![b' '; cols]; rows];
        let mut i = 0;
        let mut j = 0;
        for c_i in 0..encoded_text.len() {
            grid[i][j] = encoded_text[c_i];
            j += 1;
            i += j / cols;
            j %= cols;
        }

        let mut ans = vec![];
        let mut init_j = 0;
        i = 0;
        j = init_j;
        for _ in 0.. {
            ans.push(grid[i][j]);
            i += 1;
            j += 1;
            if i == rows || j == cols {
                init_j += 1;
                j = init_j;
                i = 0;
            }
            if init_j == cols {
                break;
            }
        }

        String::from_utf8(ans).unwrap().trim_end().to_string()
    }
}
