use std::collections::HashSet;
use std::cmp::max;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.as_bytes();

        if s.is_empty() {
            return 0
        }

        let mut map: HashSet<u8> = HashSet::new();

        let mut max_len = 0;
        let mut left = 0;

        for right in 0..bytes.len() {
            while map.contains(&bytes[right]) {
                map.remove(&bytes[left]);
                left += 1;
            }

            map.insert(bytes[right]);

            max_len = max(max_len, (right - left + 1) as i32)
        }

        max_len
    }
}
