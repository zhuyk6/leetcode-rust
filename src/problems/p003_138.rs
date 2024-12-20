pub struct Solution;

impl Solution {
    /// Returns the minimum length of the string t.
    ///
    /// The string s the the concatenation of the string t
    /// and its' similar string.
    pub fn min_anagram_length(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();

        /// Returns the count of the characters in the string.
        fn count(s: &[u8]) -> [usize; 26] {
            let mut cnt = [0; 26];
            for &c in s {
                cnt[(c - b'a') as usize] += 1;
            }
            cnt
        }

        let cnt_s = count(s);

        if cnt_s.iter().filter(|&&c| c > 0).count() == 1 {
            return 1;
        }

        let mut ans = n;

        let check_quick = |slice: &[u8]| -> bool {
            let cnt_t = count(slice);
            cnt_s
                .iter()
                .zip(cnt_t.iter())
                .all(|(&cs, &ct)| cs == 0 || (ct > 0 && cs % ct == 0))
        };

        let check = |m: usize| -> bool {
            n % m == 0 && (0..n).step_by(m).all(|i| check_quick(&s[i..i + m]))
        };

        for m in (2..).take_while(|m| m * m <= n) {
            if check(m) {
                ans = m;
                break;
            }
            if n % m == 0 && check(n / m) {
                ans = n / m;
            }
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let s = "abba".to_string();
        assert_eq!(Solution::min_anagram_length(s), 2);
    }

    #[test]
    fn sample2() {
        let s = "cdef".to_string();
        assert_eq!(Solution::min_anagram_length(s), 4);
    }
}
