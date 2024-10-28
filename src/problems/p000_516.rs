pub struct Solution;

#[allow(unused, clippy::needless_range_loop)]
impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let n = s.len();
        let mut f = vec![vec![0; n]; n];
        let s = s.as_bytes();
        for i in 0..n {
            f[i][i] = 1;
        }
        for l in 2..=n {
            for i in 0..(n - l + 1) {
                let j = i + l - 1;
                f[i][j] = if s[i] == s[j] {
                    2 + f[i + 1][j - 1]
                } else {
                    f[i + 1][j].max(f[i][j - 1])
                }
            }
        }
        f[0][n - 1]
    }
}

#[test]
fn test1() {
    let s = "bbbab".to_string();
    assert_eq!(Solution::longest_palindrome_subseq(s), 4);
}

#[test]
fn test2() {
    let s = "cbbd".to_string();
    assert_eq!(Solution::longest_palindrome_subseq(s), 2);
}
