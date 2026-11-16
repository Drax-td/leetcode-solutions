impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut result = 0;
        let mut prefix_sum = 0;
        let mut num_even_prefix_sums = 1;
        let mut num_odd_prefix_sums = 0;

        for num in arr.iter() {
            prefix_sum += num;

            if prefix_sum % 2 == 0 {
                result += num_odd_prefix_sums;
                num_even_prefix_sums += 1;
            } else {
                result += num_even_prefix_sums;
                num_odd_prefix_sums += 1;
            }

            result %= MOD;
        }

        result
    }
}
