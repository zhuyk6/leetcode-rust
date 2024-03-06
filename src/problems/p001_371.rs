#[allow(unused)]
pub fn find_the_longest_substring(s: String) -> i32 {
    use std::collections::HashMap;

    let mut map: HashMap<[u8; 5], i32> = HashMap::new();
    let mut acc = [0u8; 5];

    map.insert(acc, -1);

    let mut ans = 0;

    for (i, c) in s.bytes().enumerate() {
        match c {
            b'a' => acc[0] ^= 1,
            b'e' => acc[1] ^= 1,
            b'i' => acc[2] ^= 1,
            b'o' => acc[3] ^= 1,
            b'u' => acc[4] ^= 1,
            _ => (),
        }
        map.entry(acc)
            .and_modify(|v| {
                ans = ans.max(i as i32 - *v);
            })
            .or_insert(i as i32);
    }

    ans
}

#[test]
fn example() {
    let s = "eleetminicoworoep".to_string();
    assert_eq!(find_the_longest_substring(s), 13);

    let s = "leetcodeisgreat".to_string();
    assert_eq!(find_the_longest_substring(s), 5);
}
