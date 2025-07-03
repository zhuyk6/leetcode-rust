pub struct Solution;

impl Solution {
    pub fn minimum_mountain_removals_force(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut f: Vec<usize> = vec![usize::MAX; n];
        let mut g: Vec<usize> = vec![usize::MAX; n];
        let mut ff: Vec<usize> = vec![usize::MAX; n];
        let mut gg: Vec<usize> = vec![usize::MAX; n];

        ff[0] = 0;
        for i in 1..(n - 1) {
            ff[i] = i;
            for j in 0..i {
                if nums[j] < nums[i] {
                    f[i] = f[i].min(ff[j] + i - j - 1);
                    ff[i] = ff[i].min(ff[j] + i - j - 1);
                }
            }
        }
        // println!("f: {:#?}", f);
        // println!("ff: {:#?}", ff);

        gg[n - 1] = 0;
        for i in (1..(n - 1)).rev() {
            gg[i] = n - 1 - i;
            for j in (i + 1)..n {
                if nums[j] < nums[i] {
                    g[i] = g[i].min(gg[j] + j - i - 1);
                    gg[i] = gg[i].min(gg[j] + j - i - 1);
                }
            }
        }
        // println!("g: {:#?}", g);

        (1..(n - 1))
            .map(|i| f[i].saturating_add(g[i]))
            .min()
            .unwrap() as i32
    }

    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let bisection = |x: i32, arr: &Vec<i32>| -> usize {
            let mut l = 0;
            let mut r = n - 1;
            while l < r {
                let m = (l + r) >> 1;
                match arr[m].cmp(&x) {
                    std::cmp::Ordering::Less => l = m + 1,
                    std::cmp::Ordering::Equal => return m,
                    std::cmp::Ordering::Greater => r = m,
                }
            }
            l
        };

        let mut f = vec![0; n];
        let mut len_min_end: Vec<i32> = vec![i32::MAX; n];
        len_min_end[0] = i32::MIN;
        for i in 0..n {
            let l = bisection(nums[i], &len_min_end);
            println!("i = {i}, l = {l}");
            // f[i] = i + 1 - l;
            f[i] = l;
            len_min_end[l] = len_min_end[l].min(nums[i]);
        }

        println!("f = {f:#?}");

        let mut g = vec![0; n];
        let mut len_min_end = vec![i32::MAX; n];
        len_min_end[0] = i32::MIN;

        for i in (0..n).rev() {
            let l = bisection(nums[i], &len_min_end);
            println!("i = {i}, l = {l}");
            // g[i] = n - i - l;
            g[i] = l;
            len_min_end[l] = len_min_end[l].min(nums[i]);
        }
        println!("g = {g:#?}");

        (1..(n - 1))
            .filter(|&i| f[i] > 1 && g[i] > 1)
            .map(|i| n + 1 - f[i] - g[i])
            .min()
            .unwrap() as i32
    }
}

#[test]
fn test1() {
    let nums = vec![1, 3, 1];
    assert_eq!(Solution::minimum_mountain_removals(nums), 0);
}

#[test]
fn test2() {
    let nums = vec![2, 1, 1, 5, 6, 2, 3, 1];
    assert_eq!(Solution::minimum_mountain_removals(nums), 3);
}

#[test]
fn test3() {
    let nums = vec![23, 47, 63, 72, 81, 99, 88, 55, 21, 33, 32];
    assert_eq!(Solution::minimum_mountain_removals(nums), 1);
}

#[test]
fn test4() {
    let nums = vec![4, 3, 2, 1, 1, 2, 3, 1];
    assert_eq!(Solution::minimum_mountain_removals(nums), 4);
}

#[test]
fn test5() {
    let nums = vec![100, 92, 89, 77, 74, 66, 64, 66, 64];
    assert_eq!(Solution::minimum_mountain_removals(nums), 6);
}
