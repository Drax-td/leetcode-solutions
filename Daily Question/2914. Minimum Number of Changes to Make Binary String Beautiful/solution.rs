impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let mut result: i32 = 0;
        for pair in s.as_bytes().chunks_exact(2) {
            if pair[0] != pair[1] {
                result += 1;
            }
        }
        result
    }
}
