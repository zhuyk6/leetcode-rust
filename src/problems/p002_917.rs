struct Solution;

#[allow(unused)]
impl Solution {
    pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = [0i32; 32];

        for v in nums {
            for (i, c) in cnt.iter_mut().enumerate() {
                if v & (1 << i) > 0 {
                    *c += 1;
                }
            }
        }
        cnt.into_iter()
            .enumerate()
            .filter(|(_, c)| *c >= k)
            .map(|(i, _)| 1 << i)
            .sum::<i32>()
    }
}
