impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let n_i32 = n as i32;

        let mut ans = vec![n_i32; n];
        let mut dist = n_i32;

        for i in 0..n {
            if chars[i] == c {
                dist = 0;
            } else {
                dist += 1;
            }

            ans[i] = dist;
        }

        for i in (0..n).rev() {
            if chars[i] == c {
                dist = 0;
            } else {
                dist += 1;
            }

            ans[i] = ans[i].min(dist);
        }
        ans
    }
}
