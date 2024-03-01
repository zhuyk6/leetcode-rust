struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
        let mut nums: Vec<(usize, i32)> = alice_values
            .iter()
            .zip(bob_values.iter())
            .enumerate()
            .map(|(i, (x, y))| (i, *x + *y))
            .collect();
        nums.sort_by(|(_, a), (_, b)| b.cmp(a));

        let mut score = 0;
        for (i, (idx, _)) in nums.iter().enumerate() {
            if i & 1 == 0 {
                score += alice_values[*idx];
            } else {
                score -= bob_values[*idx];
            }
        }
        score.signum()
    }
}
