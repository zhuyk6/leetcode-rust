pub struct Solution;

#[allow(unused, clippy::needless_range_loop)]
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();

        fn put(v: i32, nums: &mut [i32]) {
            if v <= 0 || v as usize > nums.len() {
                return;
            }
            let u = nums[v as usize - 1];
            nums[v as usize - 1] = v;
            if u != v {
                put(u, nums);
            }
        }

        for i in 0..n {
            let mut v = nums[i];
            nums[i] = -1;
            // put(v, &mut nums);
            loop {
                if v <= 0 || v as usize > n {
                    break;
                }
                let u = nums[v as usize - 1];
                nums[v as usize - 1] = v;
                if u == v {
                    break;
                } else {
                    v = u;
                }
            }
        }

        for i in 0..n {
            if nums[i] != i as i32 + 1 {
                return i as i32 + 1;
            }
        }

        n as i32 + 1
    }
}

#[test]
fn test1() {
    let nums = vec![1, 2, 0];
    assert_eq!(Solution::first_missing_positive(nums), 3);
}

#[test]
fn test2() {
    let nums = vec![3, 4, -1, 1];
    assert_eq!(Solution::first_missing_positive(nums), 2);
}

#[test]
fn test3() {
    let nums = vec![7, 8, 9, 11, 12];
    assert_eq!(Solution::first_missing_positive(nums), 1);
}
