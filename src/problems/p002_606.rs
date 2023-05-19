pub fn maximum_cost_substring(s: String, chars: String, vals: Vec<i32>) -> i32 {
    let mut to_val = [0; 26];
    for i in 0..26 {
        to_val[i] = i as i32 + 1;
    }

    for (c, v) in chars.chars().zip(vals.into_iter()) {
        to_val[(c as u8 - 'a' as u8) as usize] = v;
    }

    let arr: Vec<i32> = s.chars()
        .map(|c| to_val[(c as u8 - 'a' as u8) as usize])
        .collect();

    let mut ans = 0;
    let mut acc = 0;
    for v in arr {
        acc += v;
        ans = ans.max(acc);
        if acc <= 0 { acc = 0; }
    }

    ans
}

#[test]
fn example1() {
    let s = "adaa".to_string();
    let cs = "d".to_string();
    let vs = vec![-1000];
    assert_eq!(maximum_cost_substring(s, cs, vs), 2);
}
#[test]
fn example2() {
    let s = "abc".to_string();
    let cs = "abc".to_string();
    let vs = vec![-1, -1, -1];
    assert_eq!(maximum_cost_substring(s, cs, vs), 0);
}

