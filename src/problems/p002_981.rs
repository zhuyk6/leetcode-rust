pub struct Solution;

impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut counter = [[0; 51]; 26];
        let mut ans = -1;

        #[allow(clippy::needless_range_loop)]
        for idx in 0..26usize {
            let c = b'a' + idx as u8;
            let mut i = 0;
            while i < n {
                while i < n && s[i] != c {
                    i += 1;
                }
                let mut j = i + 1;
                while j < n && s[j] == c {
                    j += 1;
                }
                if i < n {
                    let m = j - i;
                    eprintln!("[{i}, {j})");
                    for t in 1..=m {
                        counter[idx][t] += m + 1 - t;
                    }
                    i = j;
                }
            }

            // count answer
            for (i, c) in counter[idx].iter().enumerate().rev() {
                if *c >= 3 {
                    ans = ans.max(i as i32);
                    break;
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
        let s = "aaaa".to_string();
        assert_eq!(Solution::maximum_length(s), 2);
    }

    #[test]
    fn sample2() {
        let s = "abcdef".to_string();
        assert_eq!(Solution::maximum_length(s), -1);
    }
}
