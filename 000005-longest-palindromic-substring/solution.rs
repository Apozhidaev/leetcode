impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut start = 0;
        let mut max_len = 1;

        let expand = |left: isize, right: isize| -> usize {
            let mut left = left;
            let mut right = right;
            while left >= 0
                && (right as usize) < n
                && chars[left as usize] == chars[right as usize]
            {
                left -= 1;
                right += 1;
            }
            (right - left - 1) as usize
        };

        for i in 0..n {
            let len = expand(i as isize, i as isize)
                .max(expand(i as isize, (i + 1) as isize));
            if len > max_len {
                max_len = len;
                start = i - (len - 1) / 2;
            }
        }

        chars[start..start + max_len].iter().collect()
    }
}
