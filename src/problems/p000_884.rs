pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
    use std::collections::HashMap;

    fn count(s: &str) -> HashMap<&str, usize> {
        let mut map = HashMap::new();
        for word in s.split(' ') {
            *map.entry(word).or_default() += 1;
        }
        map
    }

    let cnt1 = count(&s1);
    let cnt2 = count(&s2);

    let mut ans = vec![];
    for (&word, &num) in cnt1.iter() {
        if num == 1 && !cnt2.contains_key(word) {
            ans.push(word.to_string());
        }
    }
    for (&word, &num) in cnt2.iter() {
        if num == 1 && !cnt1.contains_key(word) {
            ans.push(word.to_string());
        }
    }

    ans
}

#[test]
fn example() {
    let s1 = "this apple is sweet".to_string();
    let s2 = "this apple is sour".to_string();
    assert_eq!(
        uncommon_from_sentences(s1, s2),
        vec!["sweet".to_string(), "sour".to_string()]
    );
}
