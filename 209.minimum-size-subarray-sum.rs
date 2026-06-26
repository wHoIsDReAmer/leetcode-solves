impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        // nums.len() <= 100_000
        // nums[i] <= 10_000
        //

        let mut min_length = 100_001i32;
        let mut left = 0;
        let mut total = 0;

        for right in 0..nums.len() {
            total += nums[right];

            while target <= total {
                min_length = min_length.min((right - left + 1) as i32);

                total -= nums[left];
                left += 1;
            }
        }

        if min_length == 100_001 {
            0
        } else {
            min_length
        }
    }
}
