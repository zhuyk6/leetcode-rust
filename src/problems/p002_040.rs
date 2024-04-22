struct Solution;

// count how many numbers are equal or smaller than v
#[allow(clippy::needless_range_loop)]
fn count_lower(a: &[i32], b: &[i32], v: i64) -> usize {
    let n = a.len();
    let m = b.len();
    if n == 0 || m == 0 {
        return 0;
    }
    let mut j = m - 1;
    let mut ans = 0;
    for i in 0..n {
        while j > 0 && a[i] as i64 * b[j] as i64 > v {
            j -= 1;
        }
        if a[i] as i64 * b[j] as i64 <= v {
            ans += j + 1;
        }
    }
    ans
}

// count how many numbers are equal or higher than v
#[allow(clippy::needless_range_loop)]
fn count_higher(a: &[i32], b: &[i32], v: i64) -> usize {
    let n = a.len();
    let m = b.len();
    if n == 0 || m == 0 {
        return 0;
    }
    let mut j = 0;
    let mut ans = 0;
    for i in (0..n).rev() {
        while j < m && (a[i] as i64 * b[j] as i64) < v {
            j += 1;
        }
        if j < m {
            ans += m - j;
        }
    }
    ans
}

#[allow(dead_code)]
impl Solution {
    pub fn kth_smallest_product(mut nums1: Vec<i32>, mut nums2: Vec<i32>, k: i64) -> i64 {
        let k = k as usize;
        let p1 = nums1.partition_point(|&v| v < 0);
        let p2 = nums2.partition_point(|&v| v < 0);
        println!("p1 = {p1}, p2 = {p2}");

        if p1 > 0 {
            let mut i = 0;
            while i < p1 - 1 - i {
                nums1.swap(i, p1 - 1 - i);
                i += 1;
            }
            for v in &mut nums1[..p1] {
                *v = -*v;
            }
        }
        if p2 > 0 {
            let mut i = 0;
            while i < p2 - 1 - i {
                nums2.swap(i, p2 - 1 - i);
                i += 1;
            }
            for v in &mut nums2[..p2] {
                *v = -*v;
            }
        }

        let a = &nums1[..p1];
        let b = &nums1[p1..];
        let c = &nums2[..p2];
        let d = &nums2[p2..];

        // println!("a = {a:?}");
        // println!("c = {c:?}");

        if a.len() * d.len() + b.len() * c.len() >= k {
            // negtive
            let mut l = 0;
            let mut r = i64::MAX;
            let mut ans = 0;

            while l < r {
                let mid = (l + r) >> 1;
                let c = count_higher(a, d, mid) + count_higher(b, c, mid);
                // println!("v = {mid}, c = {c}");
                if c >= k {
                    l = mid + 1;
                    ans = mid;
                } else {
                    r = mid;
                }
            }

            -ans
        } else {
            // positive
            let k = k - a.len() * d.len() - b.len() * c.len();

            let mut l = 0;
            let mut r = i64::MAX;

            while l < r {
                let mid = (l + r) >> 1;
                let c = count_lower(b, d, mid) + count_lower(a, c, mid);
                println!("v = {mid}, c = {c}");
                if c >= k {
                    r = mid;
                } else {
                    l = mid + 1;
                }
            }

            l
        }
    }
}

#[test]
fn test1() {
    let nums1 = vec![2, 5];
    let nums2 = vec![3, 4];
    let k = 2;
    assert_eq!(Solution::kth_smallest_product(nums1, nums2, k), 8);
}

#[test]
fn test2() {
    let nums1 = vec![-4, -2, 0, 3];
    let nums2 = vec![2, 4];
    let k = 6;
    assert_eq!(Solution::kth_smallest_product(nums1, nums2, k), 0);
}

#[test]
fn test3() {
    let nums1 = vec![-2, -1, 0, 1, 2];
    let nums2 = vec![-3, -1, 2, 4, 5];
    let k = 3;
    assert_eq!(Solution::kth_smallest_product(nums1, nums2, k), -6);
}
