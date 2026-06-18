impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0
        }

        let mut max = 0;

        let mut left = 0;
        let mut right = height.len()-1;

        while left < right {
            let w = right - left;
            let h = height[left].min(height[right]);

            let extent = w * h as usize;

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }

            if max < extent { max = extent }
        }

        max as i32
    }
}
