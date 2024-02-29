struct Solution;

const N: usize = 32 + 15;
type Counter = [usize; N];

impl Solution {
    #[allow(dead_code)]
    pub fn maximum_or(nums: Vec<i32>, k: i32) -> i64 {
        fn add(cnt: &mut Counter, x: i64) {
            for (i, c) in cnt.iter_mut().enumerate() {
                if x & (1 << i) > 0 {
                    *c += 1;
                }
            }
        }

        fn sub(cnt: &mut Counter, x: i64) {
            for (i, c) in cnt.iter_mut().enumerate() {
                if x & (1 << i) > 0 {
                    *c -= 1;
                }
            }
        }

        fn from_bits(cnt: &Counter) -> i64 {
            let mut acc = 0;
            for (i, c) in cnt.iter().enumerate() {
                if *c > 0 {
                    acc |= 1 << i;
                }
            }
            acc
        }

        let mut sum = [0; N];
        for &x in &nums {
            add(&mut sum, x as i64);
        }

        let mut ans = 0;
        for &x in &nums {
            let x = x as i64;
            let y = x << k;
            sub(&mut sum, x);
            add(&mut sum, y);
            ans = ans.max(from_bits(&sum));
            sub(&mut sum, y);
            add(&mut sum, x);
        }
        ans
    }
}

#[test]
fn test1() {
    let nums = vec![12, 9];
    let k = 1;
    assert_eq!(Solution::maximum_or(nums, k), 30);
}
#[test]
fn test2() {
    let nums = vec![8, 1, 2];
    let k = 2;
    assert_eq!(Solution::maximum_or(nums, k), 35);
}
