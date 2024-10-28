pub struct Solution;

impl Solution {
    pub fn min_deletion(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut last = -1;
        let mut even = true;
        for v in nums {
            if even {
                last = v;
                even = false;
            } else if v == last {
                ans += 1;
            } else {
                even = true;
            }
        }
        if !even {
            ans += 1;
        }
        ans
    }
}
