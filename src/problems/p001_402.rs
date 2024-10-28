pub struct Solution;

impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        let n = satisfaction.len();
        satisfaction.sort_unstable();

        let mut f = vec![i32::MIN; n + 1];
        f[0] = 0;
        for (i, s) in satisfaction.into_iter().enumerate() {
            for j in (1..=(i + 1)).rev() {
                f[j] = f[j].max(f[j - 1].saturating_add(s.saturating_mul(j as i32)));
            }
        }
        f.into_iter().max().unwrap()
    }
}

#[test]
fn test1() {
    let satisfaction = vec![-1, -8, 0, 5, -9];
    assert_eq!(Solution::max_satisfaction(satisfaction), 14);
}

#[test]
fn test2() {
    let satisfaction = vec![4, 3, 2];
    assert_eq!(Solution::max_satisfaction(satisfaction), 20);
}

#[test]
fn test3() {
    let satisfaction = vec![-1, -3, -9];
    assert_eq!(Solution::max_satisfaction(satisfaction), 0);
}
