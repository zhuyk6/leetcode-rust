#[allow(dead_code)]
pub fn minimize_array_value_(nums: Vec<i32>) -> i32 {
    let check = |limit: i32| -> bool {
        let mut acc = 0_i64;
        for (i, &v) in nums.iter().enumerate() {
            acc += v as i64;
            if v > limit {
                if acc > (i+1) as i64 * limit as i64 {
                    return false;
                }
            }
        }
        true
    };


    let mut l = *nums.iter().min().unwrap();
    let mut r = *nums.iter().max().unwrap();
    let mut ans = r;
    while l <= r {
        let m = (l + r) >> 1;
        if check(m) {
            ans = m;
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    ans
}

#[allow(dead_code)]
pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
    let mut ans = 0_i64;
    let mut acc = 0_i64;
    for (i, v) in nums.into_iter().enumerate() {
        acc += v as i64;
        let ave = match acc % (i+1) as i64 {
            0 => acc / (i+1) as i64,
            _ => acc / (i+1) as i64 + 1,
        };
        ans = ans.max(ave);
    }
    ans as i32
}

#[test]
fn example() {
    let nums = vec![3,7,1,6];
    assert_eq!(minimize_array_value(nums), 5);

    let nums = vec![10, 1];
    assert_eq!(minimize_array_value(nums), 10);
}
