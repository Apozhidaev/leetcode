use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_index = HashMap::new();
        let mut left = 0;
        let mut max_len = 0;

        for (right, ch) in s.chars().enumerate() {
            if let Some(&prev) = last_index.get(&ch) {
                if prev >= left {
                    left = prev + 1;
                }
            }
            last_index.insert(ch, right);
            max_len = max_len.max(right - left + 1);
        }

        max_len as i32
    }
}
