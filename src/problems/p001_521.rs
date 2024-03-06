#[allow(unused)]
pub fn closest_to_target(arr: Vec<i32>, target: i32) -> i32 {
    const M: usize = 25;

    let n = arr.len();
    let mut sum = vec![[0_usize; M]; n];

    for j in 0..M {
        if arr[0] & (1 << j) > 0 {
            sum[0][j] = 1;
        }
    }

    for i in 1..n {
        for j in 0..M {
            if arr[i] & (1 << j) > 0 {
                sum[i][j] = sum[i - 1][j] + 1;
            } else {
                sum[i][j] = sum[i - 1][j];
            }
        }
    }

    let get = |l: usize, r: usize| -> i32 {
        let mut x = 0;
        for j in 0..M {
            let cnt = match l {
                0 => sum[r][j],
                l => sum[r][j] - sum[l - 1][j],
            };
            if cnt == r - l + 1 {
                x |= 1 << j;
            }
        }
        x
    };

    let mut ans = i32::MAX;
    let mut j = 0;
    for i in 0..n {
        j = j.max(i);
        let mut acc = get(i, j);
        ans = ans.min((acc - target).abs());
        println!("i = {}, j = {}, acc = {}, ans = {}", i, j, acc, ans);
        while j < n - 1 && acc >= target {
            j += 1;
            acc &= arr[j];
            ans = ans.min((acc - target).abs());
            println!("i = {}, j = {}, acc = {}, ans = {}", i, j, acc, ans);
        }
    }
    ans
}

#[test]
fn example() {
    let arr = vec![9, 12, 3, 7, 15];
    let target = 5;
    assert_eq!(closest_to_target(arr, target), 2);

    let arr = vec![1000000, 1000000, 1000000];
    let target = 1;
    assert_eq!(closest_to_target(arr, target), 999999);

    let arr = vec![1, 2, 4, 8, 16];
    let target = 0;
    assert_eq!(closest_to_target(arr, target), 0);
}
