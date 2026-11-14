impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        if chars.is_empty() {
            return s;
        }

        let is_vowel = |c: char| matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u');

        let mut i = 0;
        let mut j = chars.len() - 1;

        while i < j {
            if !is_vowel(chars[i]) {
                i += 1;
            } else if !is_vowel(chars[j]) {
                j -= 1;
            } else {
                chars.swap(i, j);
                i += 1;
                j -= 1;
            }
        }
        chars.into_iter().collect()
    }
}
