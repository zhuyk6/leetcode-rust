#[allow(unused)]
pub fn is_possible_divide(nums: Vec<i32>, k: i32) -> bool {
    let mut nums = nums;
    nums.sort();

    let mut cnt: Vec<(i32, u32)> = Vec::with_capacity(nums.len());
    cnt.push((nums[0], 0));
    for v in nums {
        if let Some((u, c)) = cnt.last_mut() {
            if *u == v {
                *c += 1;
            } else {
                cnt.push((v, 1));
            }
        }
    }

    // println!("cnt: {:?}", cnt);

    let k = k as usize;
    let n = cnt.len();
    if n < k {
        false
    } else {
        for i in 0..n {
            if cnt[i].1 == 0 {
                continue;
            }
            let c = cnt[i].1;
            for j in i..(i + k) {
                if j >= n || (cnt[j].0 - cnt[i].0) != (j - i) as i32 || cnt[j].1 < c {
                    return false;
                } else {
                    cnt[j].1 -= c;
                }
            }
            // println!("i = {}, cnt = {:?}", i, cnt);
        }
        true
    }
}

#[test]
fn example() {
    let nums = vec![1, 2, 3, 4];
    let k = 3;
    assert!(!is_possible_divide(nums, k));

    let nums = vec![1, 2, 3, 3, 4, 4, 5, 6];
    let k = 4;
    assert!(is_possible_divide(nums, k));

    let nums = vec![3, 2, 1, 2, 3, 4, 3, 4, 5, 9, 10, 11];
    let k = 3;
    assert!(is_possible_divide(nums, k));

    let nums = vec![3, 3, 2, 2, 1, 1];
    let k = 3;
    assert!(is_possible_divide(nums, k));
}
