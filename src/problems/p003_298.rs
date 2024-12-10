pub struct Solution;

impl Solution {
    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        let cnt2: [u32; 26] = {
            let mut cnt = [0u32; 26];
            for c in word2.as_bytes() {
                cnt[(*c - b'a') as usize] += 1;
            }
            cnt
        };

        #[inline]
        fn less_or_equal(a: &[u32], b: &[u32]) -> bool {
            a.iter().zip(b).all(|(x, y)| x <= y)
        }

        let mut cnt1 = [0u32; 26];
        let mut ans = 0usize;
        let mut j = 0;
        let s = word1.as_bytes();
        let n = s.len();
        for i in 0..n {
            if j < i {
                j = i;
                cnt1.fill(0);
            }
            while j < n && !less_or_equal(&cnt2, &cnt1) {
                cnt1[(s[j] - b'a') as usize] += 1;
                j += 1;
            }
            if less_or_equal(&cnt2, &cnt1) {
                ans += n - j + 1;
            } else {
                break;
            }
            cnt1[(s[i] - b'a') as usize] -= 1;
        }
        ans as i64
    }
}
