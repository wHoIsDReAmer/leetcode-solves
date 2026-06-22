impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;

        let mut triplets = Vec::new();
        nums.sort();

        // min = 10^5
        let mut prev_outer = -100_001;

        for i in 0..nums.len() {
            if prev_outer == nums[i] { continue; }

            let target = -1 * nums[i];

            let mut left = i + 1;
            let mut prev_left = -100_001;
            let mut right = nums.len() - 1;
            let mut prev_right = -100_001;

            while left < right {
                let sum = nums[left] + nums[right];

                if sum < target {
                    left += 1;
                } else if sum > target {
                    right -= 1;
                } else if sum == target {
                    if prev_left == nums[left] {
                        left += 1;
                        continue;
                    } else if prev_right == nums[right] {
                        right -= 1;
                        continue;
                    }

                    triplets.push(vec![nums[left], nums[right], nums[i]]);

                    prev_left = nums[left];
                    prev_right = nums[right];

                    left += 1;
                    right -= 1;
                }
            }

            prev_outer = nums[i];
        }

        triplets
    }
}
