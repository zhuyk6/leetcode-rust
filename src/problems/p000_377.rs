struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let m = target as usize;

        let mut f = vec![0; m + 1];
        f[0] = 1;
        for i in 0..=m {
            for &v in &nums {
                let v = v as usize;
                if i + v <= m {
                    f[i + v] += f[i];
                }
            }
        }
        f[m] as i32
    }
}

#[test]
fn test1() {
    let nums = vec![1, 2, 3];
    let target = 4;
    assert_eq!(Solution::combination_sum4(nums, target), 7);
}

#[test]
fn test2() {
    let nums = vec![9];
    let target = 3;
    assert_eq!(Solution::combination_sum4(nums, target), 0);
}
