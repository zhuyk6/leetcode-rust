pub struct Solution;

impl Solution {
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let mut arr: Vec<(u32, i32)> = (lo..=hi)
            .map(|v| {
                let mut x = v;
                let mut steps = 0;
                while x != 1 {
                    if x % 2 == 0 {
                        x /= 2;
                    } else {
                        x = 3 * x + 1;
                    }
                    steps += 1;
                }
                (steps, v)
            })
            .collect();

        arr.sort_unstable();

        arr[k as usize - 1].1
    }
}
