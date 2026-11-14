impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();

        let mut i = 0;
        let mut j = 0;

        while i < g.len() && j < s.len() {
            if s[i] >= g[j] {
                i += 1;
            }
            j += 1;
        }
        i as i32
    }
}
