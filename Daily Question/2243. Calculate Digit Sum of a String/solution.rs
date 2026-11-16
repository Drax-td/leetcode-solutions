impl Solution {
    pub fn digit_sum(s: String, k: i32) -> String {
        let mut buffer = s;
        while buffer.as_bytes().len() > k as usize {
            let mut sums = String::new();

            for group in buffer.as_bytes().chunks(k as usize) {
                let sum: u32 = group.iter().map(|&digit| (digit - b'0') as u32).sum();
                sums += &sum.to_string();
            }

            buffer.clear();
            buffer = sums;
        }

        buffer
    }
}
