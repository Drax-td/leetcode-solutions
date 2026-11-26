impl Solution {
    pub fn can_alice_win(mut n: i32) -> bool {
        let mut i: i32 = 10;
        while n - i >= 0 {
            n -= i;
            i -= 1;
        }
        i % 2 != 0
    }
}
