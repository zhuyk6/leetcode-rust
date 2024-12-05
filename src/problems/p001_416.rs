pub struct Solution;

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn number_of_arrays(s: String, k: i32) -> i32 {
        let s = s.as_str();
        let n = s.len();

        let mut dp = vec![0; n];

        for i in 0..n {
            for j in (i.saturating_sub(10)..=i).rev() {
                if &s[j..j + 1] == "0" {
                    continue;
                }
                match s[j..=i].parse::<i32>() {
                    Ok(v) => {
                        if v > k {
                            break;
                        } else if v > 0 {
                            dp[i] = (dp[i] + if j > 0 { dp[j - 1] } else { 1 }) % MOD;
                        }
                    }
                    Err(_) => break,
                }
            }
        }

        dp[n - 1]
    }
}

#[test]
fn test_parse() {
    let s = "1234";
    let slice = &s[1..];
    assert_eq!(slice.parse::<i32>().unwrap(), 234);

    let s = "1000000000";
    assert_eq!(s.parse::<i32>().unwrap(), 1_000_000_000);

    let s = "9876543210";
    assert!(s.parse::<i32>().is_err());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let s = "1000".to_string();
        let k = 10000;
        assert_eq!(Solution::number_of_arrays(s, k), 1);
    }

    #[test]
    fn sample2() {
        let s = "1000".to_string();
        let k = 10;
        assert_eq!(Solution::number_of_arrays(s, k), 0);
    }

    #[test]
    fn sample3() {
        let s = "1317".to_string();
        let k = 2000;
        assert_eq!(Solution::number_of_arrays(s, k), 8);
    }

    #[test]
    fn sample4() {
        let s = "2020".to_string();
        let k = 30;
        assert_eq!(Solution::number_of_arrays(s, k), 1);
    }

    #[test]
    fn sample5() {
        let s = "1234567890".to_string();
        let k = 90;
        assert_eq!(Solution::number_of_arrays(s, k), 34);
    }

    #[test]
    fn time_limit_error() {
        let s = "100000000000000000000000000".to_string();
        let k = 10;
        assert_eq!(Solution::number_of_arrays(s, k), 0);
    }
}
