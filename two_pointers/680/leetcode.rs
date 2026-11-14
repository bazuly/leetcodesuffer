impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let bytes = s.as_bytes();
        let n = bytes.len();

        fn is_palindrome(bytes: &[u8], mut left: usize, mut right: usize) -> bool {
            while left < right {
                if bytes[left] != bytes[right] {
                    return false;
                }
                left += 1;
                right -= 1;
            }
            true
        }

        let mut left = 0;
        let mut right = n - 1;

        while left < right {
            if bytes[left] != bytes[right] {
                return is_palindrome(bytes, left + 1, right)
                    || is_palindrome(bytes, left, right - 1);
            }
            left += 1;
            right -= 1;
        }
        true
    }
}
