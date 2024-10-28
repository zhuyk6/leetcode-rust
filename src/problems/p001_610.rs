pub struct Solution;

use std::f64::consts::PI;

const EPS: f64 = 1e-5;

impl Solution {
    pub fn visible_points(points: Vec<Vec<i32>>, angle: i32, location: Vec<i32>) -> i32 {
        let mut zero_num = 0;

        let mut thetas: Vec<f64> = Vec::new();
        for p in points {
            if p == location {
                zero_num += 1;
            } else {
                let v: (f64, f64) = ((p[0] - location[0]) as f64, (p[1] - location[1]) as f64);
                thetas.push(v.1.atan2(v.0));
            }
        }

        thetas.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let w: f64 = angle as f64 / 360.0 * 2.0 * PI;

        println!("arr: {:?}", thetas);
        println!("w = {w}");

        let mut max_num = 0;
        let n = thetas.len();
        let mut i = 0;
        let mut j = 0;
        while i < n {
            while j + 1 < n && (thetas[j + 1] - thetas[i] - w) < EPS
                || j + 1 < 2 * n
                    && j + 1 < i + n
                    && thetas[(j + 1) % n] - thetas[i] + 2.0 * PI - w < EPS
            {
                j += 1;
            }
            if j >= i {
                max_num = max_num.max(j - i + 1);
            } else {
                max_num = max_num.max((n + j) - i + 1);
            }
            println!("i = {i}, j = {j}, max = {max_num}");
            if i == j {
                j += 1;
            }
            i += 1;
        }

        (zero_num + max_num) as i32
    }
}

#[test]
fn test1() {
    let points = vec![vec![2, 1], vec![2, 2], vec![3, 3]];
    let angle = 90;
    let location = vec![1, 1];

    assert_eq!(Solution::visible_points(points, angle, location), 3);
}

#[test]
fn test2() {
    let points = vec![
        vec![1, 1],
        vec![2, 2],
        vec![3, 3],
        vec![4, 4],
        vec![1, 2],
        vec![2, 1],
    ];
    let angle = 0;
    let location = vec![1, 1];

    assert_eq!(Solution::visible_points(points, angle, location), 4);
}

#[test]
fn test3() {
    let points = vec![vec![0, 0], vec![0, 2]];
    let angle = 90;
    let location = vec![1, 1];

    assert_eq!(Solution::visible_points(points, angle, location), 2);
}
