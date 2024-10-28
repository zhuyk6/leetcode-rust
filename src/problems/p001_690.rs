pub struct Solution;

impl Solution {
    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let mut f: Vec<Vec<i32>> = vec![vec![0; n]; n];

        let is_alice = |l: usize, r: usize| -> bool { (r - l + 1) % 2 == n % 2 };

        let mut prefix_sum = vec![0; n];
        prefix_sum[0] = stones[0];
        for i in 1..n {
            prefix_sum[i] = prefix_sum[i - 1] + stones[i];
        }

        let sum = |l: usize, r: usize| -> i32 {
            if l == 0 {
                prefix_sum[r]
            } else {
                prefix_sum[r] - prefix_sum[l - 1]
            }
        };

        for l in 2..=n {
            for i in 0..=(n - l) {
                let j = i + l - 1;
                f[i][j] = match is_alice(i, j) {
                    true => (f[i + 1][j] + sum(i + 1, j)).max(f[i][j - 1] + sum(i, j - 1)),
                    false => (f[i + 1][j] - sum(i + 1, j)).min(f[i][j - 1] - sum(i, j - 1)),
                }
            }
        }
        f[0][n - 1]
    }
}

#[test]
fn test1() {
    let stones = vec![5, 3, 1, 4, 2];
    assert_eq!(Solution::stone_game_vii(stones), 6);
}

#[test]
fn test2() {
    let stones = vec![7, 90, 5, 1, 100, 10, 10, 2];
    assert_eq!(Solution::stone_game_vii(stones), 122);
}
