pub struct Solution;

impl Solution {
    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        // sort the positions in ascending order
        position.sort_unstable();

        /// Returns the maximum number of balls which can be placed in the boxes,
        /// such that the distance between any two balls is at least `dist`.
        fn calc(positions: &[i32], dist: i32, dp: &mut [u32]) -> u32 {
            assert!(dist >= 0);
            let n = positions.len();
            if dist == 0 {
                return n as u32;
            }

            let mut l = 0;
            for r in 0..n {
                while positions[r] - positions[l] >= dist {
                    l += 1;
                }
                dp[r] = if l == 0 { 1 } else { dp[l - 1] + 1 };
            }
            dp.iter().copied().max().unwrap()
        }

        assert!(2 <= m && m <= position.len() as i32);
        let mut lo = position.windows(2).map(|w| w[1] - w[0]).min().unwrap();
        let mut hi = position.last().unwrap() - position.first().unwrap();
        if m == 2 {
            return hi;
        } else if m == position.len() as i32 {
            return lo;
        }

        let mut ans = lo;
        let mut dp = vec![0; position.len()];
        while lo < hi {
            let mid = (lo + hi) >> 1;
            if calc(&position, mid, &mut dp) >= m as u32 {
                ans = mid;
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let position = vec![1, 2, 3, 4, 7];
        let m = 3;
        assert_eq!(Solution::max_distance(position, m), 3);
    }

    #[test]
    fn sample2() {
        let position = vec![5, 4, 3, 2, 1, 1000000000];
        let m = 2;
        assert_eq!(Solution::max_distance(position, m), 999999999);
    }
}
