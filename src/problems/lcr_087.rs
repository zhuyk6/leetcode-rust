pub struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        fn check_num(s: &[u8]) -> bool {
            if s.is_empty() || s.len() > 3 {
                return false;
            }
            if s[0] == b'0' && s.len() > 1 {
                return false;
            }
            let mut x = 0;
            for c in s.iter() {
                x = x * 10 + (c - b'0') as i32;
            }
            (0..256).contains(&x)
        }
        fn check(s: &[u8], his: &[usize]) -> Option<String> {
            if !check_num(&s[..his[0]])
                || !check_num(&s[his[0]..his[1]])
                || !check_num(&s[his[1]..his[2]])
                || !check_num(&s[his[2]..])
            {
                None
            } else {
                let mut ans = String::new();
                ans.push_str(String::from_utf8_lossy(&s[..his[0]]).as_ref());
                ans.push('.');
                ans.push_str(String::from_utf8_lossy(&s[his[0]..his[1]]).as_ref());
                ans.push('.');
                ans.push_str(String::from_utf8_lossy(&s[his[1]..his[2]]).as_ref());
                ans.push('.');
                ans.push_str(String::from_utf8_lossy(&s[his[2]..]).as_ref());
                Some(ans)
            }
        }

        fn dfs(s: &[u8], x: usize, left: usize, his: &mut [usize], ans: &mut Vec<String>) {
            if left == 0 {
                if let Some(ip) = check(s, his) {
                    ans.push(ip);
                }
                return;
            }
            if x < s.len() {
                his[3 - left] = x + 1;
                dfs(s, x + 1, left - 1, his, ans);
            }
            if x + 1 < s.len() {
                his[3 - left] = x + 2;
                dfs(s, x + 2, left - 1, his, ans);
            }
            if x + 2 < s.len() {
                his[3 - left] = x + 3;
                dfs(s, x + 3, left - 1, his, ans);
            }
        }

        let mut ans = vec![];
        let mut his = [0; 3];
        dfs(s.as_bytes(), 0, 3, &mut his, &mut ans);
        ans
    }
}

#[test]
fn test1() {
    let s = "25525511135".to_string();
    let mut ans = vec!["255.255.11.135".to_string(), "255.255.111.35".to_string()];
    let mut ret = Solution::restore_ip_addresses(s);
    ans.sort();
    ret.sort();
    assert_eq!(ret, ans);
}

#[test]
fn test2() {
    let s = "0000".to_string();
    let mut ans = vec!["0.0.0.0".to_string()];
    let mut ret = Solution::restore_ip_addresses(s);
    ans.sort();
    ret.sort();
    assert_eq!(ret, ans);
}

#[test]
fn test3() {
    let s = "010010".to_string();
    let mut ans = vec!["0.10.0.10".to_string(), "0.100.1.0".to_string()];
    let mut ret = Solution::restore_ip_addresses(s);
    ans.sort();
    ret.sort();
    assert_eq!(ret, ans);
}

#[test]
fn test4() {
    let s = "10203040".to_string();
    let mut ans = vec![
        "10.20.30.40".to_string(),
        "102.0.30.40".to_string(),
        "10.203.0.40".to_string(),
    ];
    let mut ret = Solution::restore_ip_addresses(s);
    ans.sort();
    ret.sort();
    assert_eq!(ret, ans);
}
