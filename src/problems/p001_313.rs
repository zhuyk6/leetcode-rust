struct Solution;

#[allow(unused)]
impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut a: Vec<i32> = Vec::new();

        for w in nums.chunks(2) {
            for _ in 0..w[0] {
                a.push(w[1]);
            }
        }

        a
    }
}
