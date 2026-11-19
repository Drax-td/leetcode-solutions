impl Solution {
    pub fn minimize_sum(nums: Vec<i32>) -> i32 {
        let mut sorted = nums;
        let n = sorted.len();
        let (mut fst_range, mut snd_range, mut thd_range) = (0, 0, 0);

        sorted.sort();

        fst_range = sorted[n - 1] - sorted[2];
        snd_range = sorted[n - 2] - sorted[1];
        thd_range = sorted[n - 3] - sorted[0];

        return fst_range.min(snd_range).min(thd_range);
    }
}
