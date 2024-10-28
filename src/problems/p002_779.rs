pub struct Solution;

impl Solution {
    pub fn maximum_beauty_method1(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();

        let n = nums.len();
        let mut ans = 0;

        let mut i = 0;
        for j in 0..n {
            while i <= j && nums[i] + k < nums[j] - k {
                i += 1;
            }
            assert!(i <= j);
            ans = ans.max(j - i + 1);
        }

        ans as i32
    }

    pub fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
        let m: i32 = *nums.iter().max().unwrap();

        let mut diff = vec![0; m as usize + 1];

        for v in nums {
            let l = 0.max(v - k);
            let r = v + k + 1;

            diff[l as usize] += 1;
            if r <= m {
                diff[r as usize] -= 1;
            }
        }

        let mut ans = 0;
        let mut acc = 0;
        for v in diff {
            acc += v;
            ans = ans.max(acc);
        }
        ans
    }
}
