use std::{collections::HashMap, hash::Hash};

pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
    let mut cnt = HashMap::new();
    for x in barcodes {
        *cnt.entry(x).or_insert(0usize) += 1;
    }
    let mut v: Vec<(i32, usize)> = cnt.into_iter().collect();
    v.sort_by_key(|x| x.1);
    let (x0, c0) = v.pop().unwrap();
    let mut arrs = vec![vec![x0]; c0];

    while let Some((x, c)) = v.last() {
        if *c < c0 { break; }
        for arr in &mut arrs {
            arr.push(*x);
        }
        v.pop();
    }

    let mut idx = 0usize;
    for (x, c) in v {
        for _ in 0..c {
            arrs[idx].push(x);
            idx = (idx + 1) % (c0-1);
        }
    }
    let mut ans = Vec::new();
    for arr in arrs {
        ans.extend(arr)
    }
    ans
}

fn check(arr: &Vec<i32>) -> bool {
    arr.windows(2)
        .all(|w| w[0] != w[1])
}


#[test]
fn example() {
    let arr = vec![1,1,1,2,2,2];
    let ans = rearrange_barcodes(arr);
    assert!(check(&ans));

    let arr = vec![1,1,1,1,2,2,3,3,];
    let ans = rearrange_barcodes(arr);
    assert!(check(&ans));
}
