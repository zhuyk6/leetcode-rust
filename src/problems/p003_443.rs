pub struct Solution;

impl Solution {
    pub fn max_distance(s: String, k: i32) -> i32 {
        fn calc(s: &str, mut k: i32, direction: [char; 2]) -> i32 {
            let mut max_distance = 0;
            let mut cur_dist = 0;

            for act in s.chars() {
                if direction.contains(&act) {
                    cur_dist += 1;
                } else if k > 0 {
                    cur_dist += 1;
                    k -= 1;
                } else {
                    cur_dist -= 1;
                }
                max_distance = max_distance.max(cur_dist);
            }

            max_distance
        }

        [['N', 'E'], ['E', 'S'], ['S', 'W'], ['W', 'N']]
            .iter()
            .map(|&dir| calc(&s, k, dir))
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let s = "NWSE".to_string();
        let k = 1;
        assert_eq!(Solution::max_distance(s, k), 3);
    }

    #[test]
    fn sample2() {
        let s = "NSWWEW".to_string();
        let k = 3;
        assert_eq!(Solution::max_distance(s, k), 6);
    }
}
