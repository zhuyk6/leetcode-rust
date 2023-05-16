
fn group<T: Ord>(v: Vec<T>) -> Vec<(T, u32)> {
    let mut v = v;
    let mut cnt: Vec<(T, u32)> = Vec::new();
    for val in v {
        if let Some((u, c)) = cnt.last_mut() {
            if *u == val {
                *c += 1
            } else {
                cnt.push((val, 1));
            }
        } else {
            cnt.push((val, 1));
        }
    }
    cnt
}

pub fn maximize_win(prize_positions: Vec<i32>, k: i32) -> i32 {
    let cnt = group(prize_positions);
    let n = cnt.len();

    println!("cnt: {:?}", cnt);

    // f[i]: start i 
    let mut f = vec![0; n];
    // g[i]: end i
    let mut g = vec![0; n];

    { // calculate f
        let mut acc = 0;
        let mut j = n-1;
        for i in (0..n).rev() {
            while cnt[j].0 - cnt[i].0 > k {
                acc -= cnt[j].1;
                j -= 1;
            }
            acc += cnt[i].1;
            f[i] = acc;
        }
    }
    { // calculate g
        let mut acc = 0;
        let mut i = 0;
        for j in 0..n {
            while cnt[j].0 - cnt[i].0 > k {
                acc -= cnt[i].1;
                i += 1;
            }
            acc += cnt[j].1;
            g[j] = acc;
        }
    }

    println!("f: {:?}", f);
    println!("g: {:?}", g);

    let mut ans = 0;
    let mut max = 0;
    for i in 0..n {
        ans = ans.max(max + f[i]);
        max = max.max(g[i]);
    }
    
    ans as i32
}

#[test]
fn example() {
    let v = vec![1,1,2,2,3,3,5];
    let k = 2;
    assert_eq!(maximize_win(v, k), 7);

    let v = vec![1,2,3,4, 7, 7];
    let k = 0;
    assert_eq!(maximize_win(v, k), 3);
}

