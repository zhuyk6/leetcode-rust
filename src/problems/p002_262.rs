struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn appeal_sum(s: String) -> i64 {
        let n = s.len() as i64; // length of the string
        let mut last_pos: [i64; 26] = [-1; 26];
        let mut ans = 0;
        for (i, c) in s.bytes().enumerate() {
            let ch = (c - b'a') as usize;
            let i = i as i64;
            let j = last_pos[ch];
            ans += (i - j) * (n - i);
            last_pos[ch] = i;
            println!("i={i}, j={j}, c={ch}, ans={ans}");
        }

        ans
    }
}

#[test]
fn test1() {
    let s = "abbca".into();
    assert_eq!(Solution::appeal_sum(s), 28);
}

#[test]
fn test2() {
    let s = "code".into();
    assert_eq!(Solution::appeal_sum(s), 20);
}
