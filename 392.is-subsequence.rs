impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let b1 = s.as_bytes();
        let b2 = t.as_bytes();

        if b1.is_empty() {
            return true
        }

        let mut p1 = 0;

        // is s in t
        for i in 0..b2.len() {
            if b1[p1] == b2[i] {
                p1 += 1
            }

            if p1 == b1.len() {
                return true
            }
        }

        return false
    }
}
