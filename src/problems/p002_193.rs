pub struct Solution;

impl Solution {
    pub fn min_moves_to_make_palindrome(s: String) -> i32 {
        let mut arr = s.into_bytes();
        let n = arr.len();
        let mut ans = 0;

        let mut i = 0;
        let mut last = n - 1;
        let mut mid = None;
        while i <= last {
            // arr[i..=last]
            let mut p = None;
            for j in (i + 1..=last).rev() {
                if arr[j] == arr[i] {
                    p = Some(j);
                    break;
                }
            }
            if let Some(j) = p {
                for k in j..last {
                    arr.swap(k, k + 1);
                    ans += 1;
                }
                i += 1;
                last -= 1;
            } else {
                mid = Some(i);
                i += 1;
            }
        }
        dbg!(mid);
        if let Some(p) = mid {
            ans += n / 2 - p;
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let s = "aabb".to_string();
        assert_eq!(Solution::min_moves_to_make_palindrome(s), 2);
    }

    #[test]
    fn sample2() {
        let s = "letelt".to_string();
        assert_eq!(Solution::min_moves_to_make_palindrome(s), 2);
    }

    #[test]
    fn sample3() {
        let s = "acbddab".to_string();
        // acbddab
        // abcddab
        // abdcdab
        // abdcdba
        assert_eq!(Solution::min_moves_to_make_palindrome(s), 3);
    }

    #[test]
    fn sample4() {
        let s = "abddacb".to_string();
        // abddacb
        // abddcba + 2
        // abddcba + 0
        // abdcdba + 1
        assert_eq!(Solution::min_moves_to_make_palindrome(s), 3);
    }
}
