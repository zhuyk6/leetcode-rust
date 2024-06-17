struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        fn is_substring(a: &str, b: &str) -> bool {
            let a = a.as_bytes();
            let b = b.as_bytes();
            let n = a.len();

            let mut i = 0;
            for b_c in b {
                if a[i] == *b_c {
                    i += 1;
                    if i >= n {
                        break;
                    }
                }
            }
            i >= n
        }

        let mut ans = -1;
        for (i, s) in strs.iter().enumerate() {
            if strs
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .all(|(_, t)| !is_substring(s, t))
            {
                ans = ans.max(s.len() as i32);
            }
        }
        ans
    }
}
