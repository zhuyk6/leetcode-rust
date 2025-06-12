pub struct Solution;

impl Solution {
    fn solve(s: &str, k: usize, a: char, b: char) -> i32 {
        let prefix_sum: Vec<[i32; 2]> = {
            let mut sum = vec![];
            let mut cnt = [0; 2];
            for c in s.chars() {
                if c == a {
                    cnt[0] += 1;
                } else if c == b {
                    cnt[1] += 1;
                }
                sum.push(cnt);
            }
            sum
        };

        let sum = |l: usize, r: usize, t: usize| -> i32 {
            if l == 0 {
                prefix_sum[r][t]
            } else {
                prefix_sum[r][t] - prefix_sum[l - 1][t]
            }
        };

        let n = prefix_sum.len();

        let mut left: usize = 0;
        let mut min_l = [[i32::MAX; 2]; 2];
        let mut ans = i32::MIN;
        for r in 0..n {
            while r - left + 1 >= k && sum(left, r, 0) > 0 && sum(left, r, 1) > 0 {
                if left == 0 {
                    min_l[0][0] = 0;
                } else {
                    let s = prefix_sum[left - 1];
                    let p = if s[0] % 2 == 0 { 0 } else { 1 };
                    let q = if s[1] % 2 == 0 { 0 } else { 1 };
                    min_l[p][q] = min_l[p][q].min(s[0] - s[1]);
                }
                left += 1;
            }
            let s = prefix_sum[r];
            let p = if s[0] % 2 == 0 { 0 } else { 1 };
            let q = if s[1] % 2 == 0 { 0 } else { 1 };
            ans = ans.max((s[0] - s[1]).saturating_sub(min_l[p ^ 1][q]));
        }

        ans
    }

    pub fn max_difference(s: String, k: i32) -> i32 {
        let k = k as usize;
        assert!(s.chars().all(|c| ('0'..='4').contains(&c)));

        let mut ans = i32::MIN;
        for a in '0'..='4' {
            for b in '0'..='4' {
                if a != b {
                    ans = ans.max(Solution::solve(&s, k, a, b));
                }
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
        let s = "12233".to_string();
        let k = 4;
        assert_eq!(Solution::max_difference(s, k), -1);
    }

    #[test]
    fn sample2() {
        let s = "1122211".to_string();
        let k = 4;
        assert_eq!(Solution::max_difference(s, k), 1);
    }

    #[test]
    fn sample3() {
        let s = "110".to_string();
        let k = 3;
        assert_eq!(Solution::max_difference(s, k), -1);
    }
}
