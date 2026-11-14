impl Solution {
    pub fn is_strobogrammatic(num: String) -> bool {
        fn flip(c: u8) -> Option<u8> {
            match c {
                b'0' => Some(b'0'),
                b'1' => Some(b'1'),
                b'8' => Some(b'8'),
                b'6' => Some(b'9'),
                b'9' => Some(b'6'),
                _ => None,
            }
        }

        let bytes = num.as_bytes();
        let mut i = 0;
        let mut j = bytes.len().saturating_sub(1);

        while i <= j {
            let Some(expected) = flip(bytes[i]) else {
                return false;
            };
            if expected != bytes[j] {
                return false;
            }
            i += 1;
            j = j.saturating_sub(1);
        }

        true
    }
}
