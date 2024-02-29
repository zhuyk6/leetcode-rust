struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn min_increments(n: i32, cost: Vec<i32>) -> i32 {
        let n = n as usize;

        fn dfs(n: usize, x: usize, ans: &mut i32, cost: &[i32]) -> i32 {
            if x * 2 > n {
                cost[x - 1]
            } else {
                let ls = dfs(n, x << 1, ans, cost);
                let rs = dfs(n, (x << 1) | 1, ans, cost);
                *ans += ls.max(rs) - ls.min(rs);
                ls.max(rs) + cost[x - 1]
            }
        }

        let mut ans = 0;
        dfs(n, 1, &mut ans, &cost[..]);
        ans
    }
}

#[test]
fn test1() {
    let n = 7;
    let cost = vec![1, 5, 2, 2, 3, 3, 1];
    assert_eq!(Solution::min_increments(n, cost), 6);
}
