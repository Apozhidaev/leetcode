impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (a, b) = if nums1.len() > nums2.len() {
            (nums2, nums1)
        } else {
            (nums1, nums2)
        };

        let m = a.len() as i32;
        let n = b.len() as i32;
        let half = (m + n + 1) / 2;
        let mut left = 0;
        let mut right = m;

        while left <= right {
            let i = (left + right) / 2;
            let j = half - i;

            let max_left1 = if i == 0 { i32::MIN } else { a[(i - 1) as usize] };
            let min_right1 = if i == m { i32::MAX } else { a[i as usize] };
            let max_left2 = if j == 0 { i32::MIN } else { b[(j - 1) as usize] };
            let min_right2 = if j == n { i32::MAX } else { b[j as usize] };

            if max_left1 <= min_right2 && max_left2 <= min_right1 {
                if (m + n) % 2 == 0 {
                    return (max_left1.max(max_left2) as f64
                        + min_right1.min(min_right2) as f64)
                        / 2.0;
                }
                return max_left1.max(max_left2) as f64;
            }

            if max_left1 > min_right2 {
                right = i - 1;
            } else {
                left = i + 1;
            }
        }

        unreachable!()
    }
}
