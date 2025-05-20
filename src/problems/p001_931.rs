pub struct Solution;

const MOD: i32 = 1_000_000_007;

fn mod_add(a: i32, b: i32) -> i32 {
    (a + b) % MOD
}

fn generate_valid_states(m: usize) -> Vec<usize> {
    let mut states = vec![];
    let max_state = 1 << (2 * m);

    let mask = (1 << (2 * m)) - 1;
    let mut mask10 = mask;
    for i in 0..m {
        mask10 ^= 1 << (2 * i);
    }

    // println!("mask: {:#b}, mask01: {:#b}", mask, mask10);

    let has_11 = |state: usize| -> bool { state & (state << 1) & mask10 != 0 };

    for state in 0..max_state {
        if has_11(state) {
            continue;
        }
        if has_11(!(state ^ (state << 2)) & (mask - 3)) {
            continue;
        }
        states.push(state);
    }
    states
}

impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;

        let mask = (1 << (2 * m)) - 1;
        let mut mask10 = mask;
        for i in 0..m {
            mask10 ^= 1 << (2 * i);
        }

        let valid_states = generate_valid_states(m);
        let num_states = valid_states.len();

        let mut dp = vec![vec![0; num_states]; n];
        dp[0].fill(1);

        for i in 1..n {
            for j in 0..num_states {
                let state = valid_states[j];
                for k in 0..num_states {
                    let prev_state = valid_states[k];

                    let tmp = !(state ^ prev_state) & mask;
                    if tmp & (tmp << 1) & mask10 == 0 {
                        dp[i][j] = mod_add(dp[i][j], dp[i - 1][k]);
                    }
                }
            }
        }

        dp[n - 1].iter().fold(0, |acc, &x| mod_add(acc, x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_valid_states() {
        let valid_states = generate_valid_states(1);
        let ans: Vec<usize> = vec![0b00, 0b01, 0b10];
        assert_eq!(valid_states, ans);

        let valid_states = generate_valid_states(2);
        let mut ans: Vec<usize> = vec![0b01_00, 0b10_00, 0b00_01, 0b10_01, 0b00_10, 0b01_10];
        ans.sort();
        assert_eq!(valid_states, ans);

        // let valid_states = generate_valid_states(5);
        // println!("valid_states len = {}", valid_states.len());
    }

    #[test]
    fn sample1() {
        let m = 1;
        let n = 1;
        assert_eq!(Solution::color_the_grid(m, n), 3);
    }

    #[test]
    fn sample2() {
        let m = 1;
        let n = 2;
        assert!(Solution::color_the_grid(m, n) == 6);
    }

    #[test]
    fn sample3() {
        let m = 5;
        let n = 5;
        assert!(Solution::color_the_grid(m, n) == 580986);
    }
}
