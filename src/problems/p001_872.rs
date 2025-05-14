pub struct Solution;

impl Solution {
    pub fn stone_game_viii(stones: Vec<i32>) -> i32 {
        let mut arr = stones;

        let mut sum = arr.iter().sum::<i32>();

        let mut f = 0i32;
        let mut f_right_min = -sum;
        let mut g_right_max = sum;

        sum -= arr.pop().expect("length of stones should be at least 2");

        for v in arr.into_iter().rev() {
            f = g_right_max;
            let g = f_right_min;

            f_right_min = f_right_min.min(f - sum);
            g_right_max = g_right_max.max(g + sum);

            sum -= v;
        }

        f
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let stones = vec![-1, 2, -3, 4, -5];
        assert_eq!(Solution::stone_game_viii(stones), 5);
    }

    #[test]
    fn sample2() {
        let stones = vec![7, -6, 5, 10, 5, -2, -6];
        assert_eq!(Solution::stone_game_viii(stones), 13);
    }

    #[test]
    fn sample3() {
        let stones = vec![-10, -12];
        assert_eq!(Solution::stone_game_viii(stones), -22);
    }
}
