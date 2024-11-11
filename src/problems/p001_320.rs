pub struct Solution;

impl Solution {
    pub fn minimum_distance(word: String) -> i32 {
        #[inline]
        fn get_pos(x: usize) -> (usize, usize) {
            (x / 6, x % 6)
        }

        #[inline]
        fn dist(x: usize, y: usize) -> i32 {
            let px = get_pos(x);
            let py = get_pos(y);
            (px.0 as i32 - py.0 as i32).abs() + (px.1 as i32 - py.1 as i32).abs()
        }

        let s = word.as_bytes();
        let n = s.len();

        let mut f = vec![[[i32::MAX; 26]; 26]; n];
        {
            let k = (s[0] - b'A') as usize;
            for i in 0..26 {
                f[0][k][i] = 0;
                f[0][i][k] = 0;
            }
        }
        for t in 1..n {
            let c = (s[t] - b'A') as usize;
            for i in 0..26 {
                for j in 0..26 {
                    if f[t - 1][i][j] < i32::MAX {
                        f[t][c][j] = f[t][c][j].min(f[t - 1][i][j] + dist(c, i));
                        f[t][i][c] = f[t][i][c].min(f[t - 1][i][j] + dist(c, j));
                    }
                }
            }
        }
        f[n - 1]
            .into_iter()
            .map(|v| v.into_iter().min())
            .min()
            .flatten()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let word = "CAKE".to_string();
        assert_eq!(Solution::minimum_distance(word), 3);
    }

    #[test]
    fn sample2() {
        let word = "HAPPY".to_string();
        assert_eq!(Solution::minimum_distance(word), 6);
    }
}
