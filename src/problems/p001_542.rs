pub fn longest_awesome(s: String) -> i32 {
    let s = s.as_bytes();

    let mut map = [None; 1 << 10];
    map[0] = Some(-1);

    let mut ans = 0;
    let mut mask = 0;
    for (i, &c) in s.iter().enumerate() {
        let code = (c - b'0') as usize;
        mask ^= 1 << code;

        // even -- same mask
        map[mask] = map[mask].map_or(Some(i as i32), |j| {
            ans = ans.max(i as i32 - j);
            Some(j)
        });
        // odd  -- 1 bit different
        for b in 0..10 {
            let diff_mask = mask ^ (1 << b);
            if let Some(j) = map[diff_mask] {
                ans = ans.max(i as i32 - j);
            }
        }

        println!("mask: {:10b}", mask);
    }

    ans
}

#[test]
fn example() {
    let s = "3242415".to_string();
    assert_eq!(longest_awesome(s), 5);

    let s = "12345678".to_string();
    assert_eq!(longest_awesome(s), 1);

    let s = "213123".to_string();
    assert_eq!(longest_awesome(s), 6);
}
