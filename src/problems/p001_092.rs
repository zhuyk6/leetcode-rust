pub struct Solution;

impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let s1 = str1.as_bytes();
        let s2 = str2.as_bytes();
        let n1 = s1.len();
        let n2 = s2.len();

        // dp[i][j] means the longest common subsequence of s1[..i] and s2[..j]
        // g[i][j] means the best dp[i][j] can be achieved by going from dp[i-1][j] or dp[i][j-1] or dp[i-1][j-1]
        let mut dp = vec![vec![0; n2]; n1];
        let mut g: Vec<Vec<(i32, i32)>> = vec![vec![(0, 0); n2]; n1];
        #[allow(clippy::needless_range_loop)]
        for i in 0..n1 {
            for j in 0..n2 {
                if s1[i] == s2[j] {
                    dp[i][j] = if i > 0 && j > 0 {
                        dp[i - 1][j - 1] + 1
                    } else {
                        1
                    };
                    g[i][j] = (i as i32 - 1, j as i32 - 1);
                } else {
                    match (i, j) {
                        (0, 0) => {
                            dp[i][j] = 0;
                            g[i][j] = (0, -1);
                        }
                        (0, _) => {
                            dp[i][j] = dp[i][j - 1];
                            g[i][j] = (0, j as i32 - 1);
                        }
                        (_, 0) => {
                            dp[i][j] = dp[i - 1][j];
                            g[i][j] = (i as i32 - 1, 0);
                        }
                        _ => {
                            if dp[i - 1][j] > dp[i][j - 1] {
                                dp[i][j] = dp[i - 1][j];
                                g[i][j] = (i as i32 - 1, j as i32);
                            } else {
                                dp[i][j] = dp[i][j - 1];
                                g[i][j] = (i as i32, j as i32 - 1);
                            };
                        }
                    };
                }
            }
        }

        // println!("dp = {:?}", dp[n1 - 1][n2 - 1]); // debug

        // reconstruct the shortest common supersequence
        let mut v: Vec<u8> = vec![];
        let mut i = n1 - 1;
        let mut j = n2 - 1;
        while g[i][j].0 >= 0 && g[i][j].1 >= 0 {
            let (x, y) = g[i][j];
            let (x, y) = (x as usize, y as usize);
            if x < i && y < j {
                v.push(s1[i]);
                i -= 1;
                j -= 1;
            } else if x < i {
                v.push(s1[i]);
                i -= 1;
            } else {
                v.push(s2[j]);
                j -= 1;
            }
        }
        let mut i = i as i32;
        let mut j = j as i32;
        if s1[i as usize] == s2[j as usize] {
            v.push(s1[i as usize]);
            i -= 1;
            j -= 1;
        }
        while i >= 0 {
            v.push(s1[i as usize]);
            i -= 1;
        }
        while j >= 0 {
            v.push(s2[j as usize]);
            j -= 1;
        }

        // reverse the v
        v.reverse();
        unsafe { String::from_utf8_unchecked(v) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let str1 = String::from("abac");
        let str2 = String::from("cab");
        let ans = "cabac".to_string();
        let ret = Solution::shortest_common_supersequence(str1, str2);
        // println!("ret = {}", ret);
        assert_eq!(ans.len(), ret.len());
    }

    #[test]
    fn sample2() {
        let str1 = String::from("aaaa");
        let str2 = String::from("aaaa");
        let ans = "aaaa".to_string();
        let ret = Solution::shortest_common_supersequence(str1, str2);
        // println!("ret = {}", ret);
        assert_eq!(ans.len(), ret.len());
    }

    #[test]
    fn issue() {
        let str1 = String::from("bbabacaa");
        let str2 = String::from("cccababab");
        let ans = "bbcccabacabab".to_string();
        let ret = Solution::shortest_common_supersequence(str1, str2);
        // println!("ret = {}", ret);
        assert_eq!(ans.len(), ret.len());
    }
}
