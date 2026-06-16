impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // not duplicated index
        let mut p1 = 0;

        let mut prev = -101;
        for p in 0..nums.len() {
            if prev != nums[p] {
                nums[p1] = nums[p];
                p1 += 1;
            }

            prev = nums[p]
        }

        p1 as i32
    }
}
