pub struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        fn dfs(row: u32, n: u32, left: u32, mid: u32, right: u32, ans: &mut u32) {
            if row == n {
                *ans += 1;
                return;
            }
            let m: u32 = (1 << n) - 1;
            let mask = left | mid | right;
            for i in 0..n {
                let b = 1 << i;
                if b & mask == 0 {
                    dfs(
                        row + 1,
                        n,
                        ((left | b) << 1) & m,
                        mid | b,
                        (right | b) >> 1,
                        ans,
                    );
                }
            }
        }
        let mut ans = 0;
        dfs(0, n as u32, 0, 0, 0, &mut ans);
        ans as i32
    }
}
