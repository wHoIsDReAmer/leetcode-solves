use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if words.is_empty() || s.is_empty() {
            return vec![];
        }

        let word_len = words[0].len();
        let total_words = words.len();

        let mut word_frequency: HashMap<String, i32> = HashMap::new();
        for w in words {
            *word_frequency.entry(w).or_insert(0) += 1;
        }

        let mut corrections = Vec::new();

        for i in 0..word_len {
            let mut cnt = 0;

            let mut c_f: HashMap<String, i32> = HashMap::new();

            let mut left = i;
            let mut right = i;

            while (right + word_len) <= s.len() {
                let word = s[right..(right + word_len)].to_string();

                if word_frequency.contains_key(&word) {
                    cnt += 1;
                    *c_f.entry(word.clone()).or_insert(0) += 1;

                    while c_f[&word] > word_frequency[&word] {
                        let l_word = s[left..(left + word_len)].to_string();

                        *c_f.get_mut(&l_word).unwrap() -= 1;
                        cnt -= 1;
                        left += word_len;
                    }

                    if cnt == total_words {
                        corrections.push(left as i32);

                        let l_word = s[left..(left + word_len)].to_string();

                        *c_f.get_mut(&l_word).unwrap() -= 1;

                        cnt -= 1;
                        left += word_len;
                    }
                } else { // if it met something wrong words
                    c_f.clear();
                    cnt = 0;
                    left = right + word_len;
                }

                right += word_len;
            }
        }

        corrections
    }
}
