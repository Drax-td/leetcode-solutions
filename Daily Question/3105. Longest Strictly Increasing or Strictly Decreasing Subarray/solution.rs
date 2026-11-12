use std::cmp::Ordering::{Equal, Greater, Less};

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let (mut a, mut b, mut c) = (1, 1, 1);

        for i in 1..nums.len() {
            match nums[i].cmp(&nums[i - 1]) {
                Less => {
                    a += 1;
                    b = 1;
                    c = c.max(a);
                }
                Greater => {
                    a = 1;
                    b += 1;
                    c = c.max(b);
                }
                Equal => {
                    a = 1;
                    b = 1;
                }
            }
        }

        return c;
    }
}
