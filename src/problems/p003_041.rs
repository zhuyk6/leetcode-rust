struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_selected_elements(mut nums: Vec<i32>) -> i32 {
        const M: usize = 1_000_000;
        nums.sort_unstable();

        use std::collections::HashMap;
        let mut g: HashMap<i32, i32> = HashMap::new();

        // let mut f = vec![0; M + 1];
        let mut ans = 1;

        // f[nums[0] as usize + 1] = 1;
        // f[nums[0] as usize] = 1;

        g.insert(nums[0] + 1, 1);
        g.insert(nums[0], 1);

        for &v in &nums[1..] {
            // f[v as usize + 1] = f[v as usize] + 1;
            // ans = ans.max(f[v as usize + 1]);
            let tmp = g.get(&v).unwrap_or(&0) + 1;
            g.insert(v + 1, tmp);
            ans = ans.max(tmp);

            // f[v as usize] = f[v as usize - 1] + 1;
            // ans = ans.max(f[v as usize]);
            let tmp = g.get(&(v - 1)).unwrap_or(&0) + 1;
            g.insert(v, tmp);
            ans = ans.max(tmp);
        }

        ans
    }
}
