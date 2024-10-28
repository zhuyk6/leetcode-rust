pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = words.len();
    let vowel = [b'a', b'e', b'i', b'o', b'u'];
    let mut word_is_ok = words.into_iter().map(|s| {
        let v: Vec<u8> = s.bytes().collect();
        match vowel.contains(v.first().unwrap()) && vowel.contains(v.last().unwrap()) {
            true => 1,
            false => 0,
        }
    });

    let mut sum = vec![0; n];
    sum[0] = word_is_ok.next().unwrap();

    for (i, v) in (1..).zip(word_is_ok) {
        sum[i] = sum[i - 1] + v;
    }

    let get = |l: usize, r: usize| -> i32 {
        match l {
            0 => sum[r],
            _ => sum[r] - sum[l - 1],
        }
    };

    queries
        .into_iter()
        .map(|q| get(q[0] as usize, q[1] as usize))
        .collect()
}

#[test]
fn example() {
    let words = ["aba", "bcb", "ece", "aa", "e"];
    let words = words.into_iter().map(|s| s.to_string()).collect();
    let queries = vec![vec![0, 2], vec![1, 4], vec![1, 1]];
    assert_eq!(vowel_strings(words, queries), vec![2, 3, 0]);
}
