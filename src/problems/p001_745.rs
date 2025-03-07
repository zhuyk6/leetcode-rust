pub struct Solution;

impl Solution {
    pub fn check_partitioning(s: String) -> bool {
        let s = s.as_bytes();
        let n = s.len();

        let mut valid: Vec<Vec<bool>> = vec![vec![false; n]; n];

        // odd
        for mid in 0..n {
            valid[mid][mid] = true;
            for len in 1..=n {
                if mid + len >= n || mid < len {
                    break;
                }
                let l = mid - len;
                let r = mid + len;
                if s[l] != s[r] {
                    break;
                }
                valid[l][r] = true;
            }
        }

        // even
        for mid1 in 0..(n - 1) {
            let mid2 = mid1 + 1;
            for len in 0..=n {
                if mid1 < len || mid2 + len >= n {
                    break;
                }
                let l = mid1 - len;
                let r = mid2 + len;
                if s[l] != s[r] {
                    break;
                }
                valid[l][r] = true;
            }
        }

        for i in 0..n {
            for j in (i + 1)..(n - 1) {
                if valid[0][i] && valid[i + 1][j] && valid[j + 1][n - 1] {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let s = "abcbdd".to_string();
        assert!(Solution::check_partitioning(s));
    }

    #[test]
    fn sample2() {
        let s = "bcbddxy".to_string();
        assert!(!Solution::check_partitioning(s));
    }
}
