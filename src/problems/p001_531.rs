pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
    #[inline]
    fn g(n: usize) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            2..=9 => 2,
            10..=99 => 3,
            100 => 4,
            _ => panic!("impossible!"),
        }
    }

    let n = s.len();
    let k = k as usize;
    let s: Vec<u8> = s.into_bytes();
    let mut f = vec![vec![i32::MAX; k + 1]; n + 1];

    f[0][0] = 0;

    // f[i] ~ s[i-1]
    for i in 1..=n {
        for j in 0..=k {
            if j > 0 {
                f[i][j] = f[i - 1][j - 1];
            }
            let mut diff = 0;
            let mut same = 0;
            for t in (1..=i).rev() {
                if s[i - 1] == s[t - 1] {
                    same += 1;
                    f[i][j] = f[i][j].min(f[t - 1][j - diff].saturating_add(g(same)));
                } else {
                    diff += 1;
                    if diff > j {
                        break;
                    }
                }
            }
            println!(
                "i = {i}, j = {j}, f[{i}][{j}] = {f}",
                i = i,
                j = j,
                f = f[i][j]
            );
        }
    }

    f[n][k]
}

#[test]
fn example() {
    let s = "aaabcccd".to_string();
    let k = 2;
    assert_eq!(get_length_of_optimal_compression(s, k), 4);

    let s = "aabbaa".to_string();
    let k = 2;
    assert_eq!(get_length_of_optimal_compression(s, k), 2);
}

#[test]
fn wrong() {
    let s = "a".to_string();
    let k = 1;
    assert_eq!(get_length_of_optimal_compression(s, k), 0);
}
