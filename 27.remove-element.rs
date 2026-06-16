impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut p1 = 0usize;

        for p in 0..nums.len() {
            if nums[p] != val {
                nums[p1] = nums[p];
                p1 += 1;
            }
        }

        p1 as i32
    }
}
