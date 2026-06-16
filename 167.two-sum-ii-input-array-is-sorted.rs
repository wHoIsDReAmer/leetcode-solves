use std::collections::HashMap;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m: HashMap<i32, usize> = HashMap::new();

        for (i, n) in numbers.iter().enumerate() {
            let c = target - n;

            if let Some(&i2) = m.get(&c) {
                return vec![(i2+1) as i32, (i+1) as i32]
            }

            m.insert(*n, i);
        }

        // unreachable
        vec![]
    }
}
