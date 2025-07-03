pub struct Solution;

impl Solution {
    pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
        let n = max_heights.len();
        let mut f: Vec<i64> = vec![0; n];
        let mut g: Vec<i64> = vec![0; n];
        let mut que: Vec<(i64, i64)> = vec![(-1, -1); n + 1];

        // from left to right
        que[0] = (i64::MIN, -1);
        let mut tail = 0;
        let mut acc = 0;
        for i in 0..n {
            while 0 < tail && que[tail].0 >= max_heights[i] as i64 {
                acc -= que[tail].0 * (que[tail].1 - que[tail - 1].1);
                tail -= 1;
            }
            acc += (i as i64 - que[tail].1) * max_heights[i] as i64;
            tail += 1;
            que[tail] = (max_heights[i] as i64, i as i64);
            f[i] = acc;
        }

        // from left to right
        que[0] = (i64::MIN, n as i64);
        let mut tail = 0;
        let mut acc = 0;
        for i in (0..n).rev() {
            while 0 < tail && que[tail].0 >= max_heights[i] as i64 {
                acc -= que[tail].0 * (que[tail - 1].1 - que[tail].1);
                tail -= 1;
            }
            acc += (que[tail].1 - i as i64) * max_heights[i] as i64;
            tail += 1;
            que[tail] = (max_heights[i] as i64, i as i64);
            g[i] = acc;
        }

        println!("{f:#?}");
        println!("{g:#?}");

        let mut ans = i64::MIN;
        for i in 0..n {
            ans = ans.max(f[i] + g[i] - max_heights[i] as i64);
        }
        ans
    }
}

#[test]
fn test1() {
    let max_heights = vec![5, 3, 4, 1, 1];
    assert_eq!(Solution::maximum_sum_of_heights(max_heights), 13);
}

#[test]
fn test2() {
    let max_heights = vec![6, 5, 3, 9, 2, 7];
    assert_eq!(Solution::maximum_sum_of_heights(max_heights), 22);
}

#[test]
fn test3() {
    let max_heights = vec![3, 2, 5, 5, 2, 3];
    assert_eq!(Solution::maximum_sum_of_heights(max_heights), 18);
}

#[test]
fn test4() {
    let max_heights = vec![100];
    assert_eq!(Solution::maximum_sum_of_heights(max_heights), 100);
}

#[test]
fn test5() {
    let max_heights = vec![1, 2, 3];
    assert_eq!(Solution::maximum_sum_of_heights(max_heights), 6);
}
