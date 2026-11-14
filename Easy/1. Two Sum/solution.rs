use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::<i32, usize>::new();

        for i in 0..nums.len() {
            if let Some(index) = map.get(&(target - nums[i])) {
                return vec![*index as i32, i as i32];
            }

            map.insert(nums[i], i);
        }

        return vec![];
    }
}
