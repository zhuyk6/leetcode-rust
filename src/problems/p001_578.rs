#[allow(unused)]
pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
    #[inline]
    fn encode(c: char) -> usize {
        match c {
            'a'..='z' => c as usize - 'a' as usize,
            _ => panic!("invalid character"),
        }
    }

    let n = colors.len();
    let mut f = vec![[i32::MAX; 27]; n];

    for (i, c) in colors.chars().enumerate() {
        let c = encode(c);
        if i == 0 {
            f[i][c] = 0;
            f[i][26] = needed_time[i];
        } else {
            // keep colors[i]
            for k in 0..=26 {
                if k != c {
                    f[i][c] = f[i][c].min(f[i - 1][k]);
                }
            }
            // delete colors[i]
            f[i][c] = f[i][c].min(f[i - 1][c].saturating_add(needed_time[i]));
            for k in 0..=26 {
                if k != c {
                    f[i][k] = f[i - 1][k].saturating_add(needed_time[i]);
                }
            }
        }
        println!("i = {}", i);
        for (j, v) in f[i].iter().enumerate() {
            if *v < i32::MAX {
                print!("({}, {})", j, v);
            }
        }
        println!();
    }
    *f[n - 1].iter().min().unwrap()
}

#[test]
fn example1() {
    let colors = "abaac".to_string();
    let needed_time = vec![1, 2, 3, 4, 5];

    assert_eq!(min_cost(colors, needed_time), 3);
}
#[test]
fn example2() {
    let colors = "abc".to_string();
    let needed_time = vec![1, 2, 3];

    assert_eq!(min_cost(colors, needed_time), 0);
}
#[test]
fn example3() {
    let colors = "aabaa".to_string();
    let needed_time = vec![1, 2, 3, 4, 1];

    assert_eq!(min_cost(colors, needed_time), 2);
}
