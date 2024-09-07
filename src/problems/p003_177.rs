struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;

        use std::collections::HashMap;
        let mut pre_pos: HashMap<i32, usize> = HashMap::default();

        let mut f = vec![vec![0; k + 1]; n];
        let mut pre_max = vec![0; k + 1];

        f[0].fill(1);
        pre_pos.insert(nums[0], 0);
        // pre_max[0] = 1;
        pre_max.fill(1);

        for i in 1..n {
            f[i][0] = 1;

            // nums[ii] == nums[i]
            if let Some(&ii) = pre_pos.get(&nums[i]) {
                eprintln!("ii = {ii}");
                for j in 0..=k {
                    f[i][j] = f[i][j].max(f[ii][j] + 1);
                }
            }
            pre_pos.insert(nums[i], i);

            // eprintln!("f[{i}] = {:?}", f[i]);

            // nums[ii] != nums[i]
            for j in 1..=k {
                f[i][j] = f[i][j].max(pre_max[j - 1] + 1);
            }
            #[allow(clippy::needless_range_loop)]
            for j in 0..k {
                pre_max[j] = pre_max[j].max(f[i][j]);
            }
            // eprintln!("pre_max = {pre_max:?}");
            // eprintln!("f[{i}] = {:?}\n\n", f[i]);
        }
        (0..n).map(|i| f[i][k]).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn sample1() {
        let nums = vec![1, 2, 1, 1, 3];
        let k = 2;
        assert_eq!(Solution::maximum_length(nums, k), 4);
    }

    #[test]
    fn sample2() {
        let nums = vec![1, 2, 3, 4, 5, 1];
        let k = 0;
        assert_eq!(Solution::maximum_length(nums, k), 2);
    }

    #[test]
    fn sample3() {
        let nums = vec![2];
        let k = 1;
        assert_eq!(Solution::maximum_length(nums, k), 1);
    }

    #[test]
    fn sample4() {
        let nums = vec![5, 1];
        let k = 2;
        assert_eq!(Solution::maximum_length(nums, k), 2);
    }

    #[test]
    fn sample5() {
        let nums = vec![5, 1, 1];
        let k = 0;
        assert_eq!(Solution::maximum_length(nums, k), 2);
    }
}
