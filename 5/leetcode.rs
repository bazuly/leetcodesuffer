impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut start = 0;
        let mut best = 1.min(n);

        let mut expand = |mut left: isize, mut right: isize| {
            while left >= 0 && right < n as isize && bytes[left as usize] == bytes[right as usize] {
                let len = (right - left + 1) as usize;
                if len > best {
                    start = left as usize;
                    best = len;
                }
                left -= 1;
                right += 1;
            }
        };

        for i in 0..n {
            expand(i as isize, i as isize); // "aba"
            expand(i as isize, i as isize + 1); // "bb"
        }

        s[start..start + best].to_string()
    }
}
