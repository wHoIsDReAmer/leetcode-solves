impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut candidate = 0;
        let mut count = 0;

        for i in nums {
            if candidate != i {
                count -= 1;

                if count <= 0 {
                    candidate = i;
                    count = 1;
                }
            } else {
                count += 1;
            }
        }

        return candidate;
    }
}
