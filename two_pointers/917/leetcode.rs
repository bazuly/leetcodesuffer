impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut right = chars.len().saturating_sub(1);

        while left < right {
            if !chars[left].is_alphabetic() {
                left += 1;
            } else if !chars[right].is_alphabetic() {
                right = right.saturating_sub(1);
            } else {
                chars.swap(left, right);
                left += 1;
                right = right.saturating_sub(1);
            }
        }
        chars.into_iter().collect()
    }
}
