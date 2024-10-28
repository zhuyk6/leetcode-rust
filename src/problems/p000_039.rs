pub struct Solution;

impl Solution {
    pub fn combination_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut f: Vec<Vec<Vec<i32>>> = vec![vec![]; target as usize + 1];

        f[0].push(vec![]);

        for v in nums {
            if v > target {
                continue;
            }
            let v = v as usize;
            for i in 0..=(target as usize - v) {
                for mut g in f[i].clone() {
                    g.push(v as i32);
                    f[i + v].push(g);
                }
            }
        }

        f[target as usize].clone()
    }
}

#[test]
fn test1() {
    let nums = vec![2, 3, 6, 7];
    let target = 7;
    let ans = vec![vec![2, 2, 3], vec![7]];
    assert_eq!(Solution::combination_sum(nums, target), ans);
}

#[test]
fn test2() {
    let nums = vec![2, 3, 5];
    let target = 8;
    let ans = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];
    assert_eq!(Solution::combination_sum(nums, target), ans);
}

#[test]
fn test3() {
    let nums = vec![2];
    let target = 1;
    let ans: Vec<Vec<i32>> = vec![];
    assert_eq!(Solution::combination_sum(nums, target), ans);
}
