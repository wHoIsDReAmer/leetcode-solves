impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();

        // considering array overflow
        let k = (k as usize) % n;

        // early return
        if k == 0 { return; }

        Self::reverse(nums, 0, n - 1);

        Self::reverse(nums, 0, k - 1);

        Self::reverse(nums, k, n - 1);
    }

    fn reverse(nums: &mut Vec<i32>, mut left: usize, mut right: usize) {
        while left < right {
            nums.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}
