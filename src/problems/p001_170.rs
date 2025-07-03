pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
    fn f(s: String) -> usize {
        let mut cnt = [0; 26];
        for b in s.into_bytes() {
            cnt[(b - b'a') as usize] += 1;
        }
        cnt.into_iter().enumerate().find(|(_, c)| *c > 0).unwrap().1
    }

    let mut words: Vec<usize> = words.into_iter().map(f).collect();

    words.sort();

    println!("words: {words:?}");

    queries
        .into_iter()
        .map(f)
        .map(|v| {
            let idx = words.partition_point(|u| *u <= v);
            println!("v: {v}, idx: {idx}");
            (words.len() - idx) as i32
        })
        .collect()
}

#[test]
fn example() {
    let queries = vec!["cbd".to_string()];
    let words = vec!["zaaaz".to_string()];
    assert_eq!(num_smaller_by_frequency(queries, words), vec![1]);

    let queries = vec!["bbb".to_string(), "cc".to_string()];
    let words = vec![
        "a".to_string(),
        "aa".to_string(),
        "aaa".to_string(),
        "aaaa".to_string(),
    ];
    assert_eq!(num_smaller_by_frequency(queries, words), vec![1, 2]);
}
