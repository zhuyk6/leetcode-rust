pub struct Solution;

impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut counter = [[-1; 3]; 26];
        let mut ans = -1;

        fn insert(cnt: &mut [i32], x: i32) {
            if x >= cnt[2] {
                cnt[0] = cnt[1];
                cnt[1] = cnt[2];
                cnt[2] = x;
            } else if x >= cnt[1] {
                cnt[0] = cnt[1];
                cnt[1] = x;
            } else if x >= cnt[0] {
                cnt[0] = x;
            }
        }

        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            while j < n && s[j] == s[i] {
                j += 1;
            }
            let m = j - i;
            let idx = (s[i] - b'a') as usize;
            // eprintln!("[{i}, {j}), char = {idx}");

            let cnt = &mut counter[idx];
            insert(cnt, m as i32);

            i = j;
        }

        for cnt in &counter {
            if cnt[2] == cnt[1] && cnt[1] == cnt[0] {
                ans = ans.max(cnt[2]);
            } else if cnt[2] > 1 && (cnt[2] == cnt[1] || cnt[2] - 1 == cnt[1]) {
                ans = ans.max(cnt[2] - 1);
            } else if cnt[2] > 2 {
                ans = ans.max(cnt[2] - 2);
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

    #[test]
    fn wrong3() {
        let s = "bbc".to_string();
        assert_eq!(Solution::maximum_length(s), -1);
    }
}
