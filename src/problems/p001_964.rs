struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_obstacle_course_at_each_position(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut f = vec![0; n];
        let mut g = vec![i32::MAX; n + 1];
        g[0] = i32::MIN;

        for (i, v) in nums.into_iter().enumerate() {
            let p = g.partition_point(|u| *u <= v);
            f[i] = p as i32;
            g[p] = g[p].min(v);
        }

        f
    }
}

#[test]
fn test1() {
    let nums = vec![1, 2, 3, 2];
    assert_eq!(
        Solution::longest_obstacle_course_at_each_position(nums),
        vec![1, 2, 3, 3]
    );
}

#[test]
fn test2() {
    let nums = vec![2, 2, 1];
    assert_eq!(
        Solution::longest_obstacle_course_at_each_position(nums),
        vec![1, 2, 1]
    );
}

#[test]
fn test3() {
    let nums = vec![3, 1, 5, 6, 4, 2];
    assert_eq!(
        Solution::longest_obstacle_course_at_each_position(nums),
        vec![1, 1, 2, 3, 2, 2]
    );
}
