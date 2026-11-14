impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let mut min_sum: i32 = -1;

        for size in l..=r {
            let min_subarrays: Option<i32> = nums
                .windows(size as usize)
                .map(|window| window.iter().sum::<i32>())
                .filter(|&sum| sum > 0)
                .min();

            if let Some(sum) = min_subarrays {
                if min_sum == -1 || sum < min_sum {
                    min_sum = sum;
                }
            }
        }

        return min_sum;
    }
}
