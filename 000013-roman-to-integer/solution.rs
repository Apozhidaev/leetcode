use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let values: HashMap<char, i32> = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        let chars: Vec<char> = s.chars().collect();
        let mut result = 0;

        for i in 0..chars.len() {
            let curr = values[&chars[i]];
            let next = chars.get(i + 1).map(|c| values[c]).unwrap_or(0);
            result += if curr < next { -curr } else { curr };
        }

        result
    }
}
