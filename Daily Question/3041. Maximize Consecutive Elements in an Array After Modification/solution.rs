impl Solution {
    pub fn max_selected_elements(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mut dp = (0, 0);
        let (mut prev, mut result) = (0, 0);

        for x in nums {
            dp = match x - prev {
                0 => (dp.0, dp.0 + 1),
                1 => (dp.0 + 1, dp.1 + 1),
                2 => (dp.1 + 1, 1),
                _ => (1, 1),
            };

            prev = x;
            result = result.max(dp.0.max(dp.1));
        }

        result
    }
}
