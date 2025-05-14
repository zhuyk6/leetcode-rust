pub struct Solution;

impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let arr = stone_value;
        let n = arr.len();

        let mut f = vec![0; n];
        let mut g = vec![0; n];

        for i in (0..n).rev() {
            let mut acc = 0;
            let mut f_max = i32::MIN;
            let mut g_min = i32::MAX;
            for j in i..i + 3 {
                if j >= n {
                    break;
                }
                acc += arr[j];

                let g = if j + 1 < n { g[j + 1] } else { 0 };
                f_max = f_max.max(g + acc);

                let f = if j + 1 < n { f[j + 1] } else { 0 };
                g_min = g_min.min(f - acc)
            }
            f[i] = f_max;
            g[i] = g_min;
        }

        match f[0].cmp(&0) {
            std::cmp::Ordering::Less => "Bob".to_string(),
            std::cmp::Ordering::Equal => "Tie".to_string(),
            std::cmp::Ordering::Greater => "Alice".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let stone_value = vec![1, 2, 3, 7];
        assert_eq!(Solution::stone_game_iii(stone_value), "Bob".to_string());
    }

    #[test]
    fn sample2() {
        let stone_value = vec![1, 2, 3, -9];
        assert_eq!(Solution::stone_game_iii(stone_value), "Alice".to_string());
    }

    #[test]
    fn sample3() {
        let stone_value = vec![1, 2, 3, 6];
        assert_eq!(Solution::stone_game_iii(stone_value), "Tie".to_string());
    }
}
