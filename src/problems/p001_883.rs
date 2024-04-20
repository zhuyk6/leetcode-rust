struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_skips(dist: Vec<i32>, speed: i32, hours_before: i32) -> i32 {
        let n = dist.len();
        let v = speed as i64;
        let hours = hours_before as i64;

        let up = |x: i64| -> i64 {
            if x == i64::MAX {
                i64::MAX
            } else if x % v == 0 {
                x
            } else {
                (x / v + 1) * v
            }
        };

        let mut f = vec![vec![i64::MAX; n]; n];

        f[0][0] = dist[0] as i64;

        for i in 1..n {
            f[i][0] = up(f[i - 1][0]).saturating_add(dist[i] as i64);
            for j in 1..n {
                let tmp1 = f[i - 1][j - 1].saturating_add(dist[i] as i64);
                let tmp2 = up(f[i - 1][j]).saturating_add(dist[i] as i64);
                // println!("{tmp1} -- {tmp2}");
                f[i][j] = i64::min(tmp1, tmp2);
            }
            // println!("f[{i}] = {:?}", f[i]);
        }

        f[n - 1]
            .iter()
            .enumerate()
            .find(|(_j, t)| **t <= v * hours)
            .map(|(j, _t)| j as i32)
            .unwrap_or(-1)
    }
}

#[test]
fn test1() {
    let dist = vec![1, 3, 2];
    let speed = 4;
    let hours_before = 2;
    assert_eq!(Solution::min_skips(dist, speed, hours_before), 1);
}

#[test]
fn test2() {
    let dist = vec![7, 3, 5, 5];
    let speed = 2;
    let hours_before = 10;
    assert_eq!(Solution::min_skips(dist, speed, hours_before), 2);
}

#[test]
fn test3() {
    let dist = vec![7, 3, 5, 5];
    let speed = 1;
    let hours_before = 10;
    assert_eq!(Solution::min_skips(dist, speed, hours_before), -1);
}
