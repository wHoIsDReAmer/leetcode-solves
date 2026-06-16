impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }

        let mut p = 2;

        for i in 2..nums.len() {
            if nums[i] != nums[p-2] {
                nums[p] = nums[i];
                p += 1;
            }
        }

        p as i32
    }
}
