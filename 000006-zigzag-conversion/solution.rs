impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        if num_rows == 1 || num_rows >= s.len() {
            return s;
        }

        let mut rows = vec![String::new(); num_rows];
        let mut row = 0isize;
        let mut step = 1isize;

        for ch in s.chars() {
            rows[row as usize].push(ch);
            if row == 0 {
                step = 1;
            } else if row == (num_rows as isize) - 1 {
                step = -1;
            }
            row += step;
        }

        rows.concat()
    }
}
