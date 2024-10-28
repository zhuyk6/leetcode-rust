pub struct Solution;

impl Solution {
    pub fn spiral_array(array: Vec<Vec<i32>>) -> Vec<i32> {
        let m = array.len() as i32;
        if m == 0 {
            return vec![];
        }
        let n = array[0].len() as i32;

        let mut vis: Vec<Vec<bool>> = vec![vec![false; n as usize]; m as usize];

        let mut ans = vec![];
        let dd = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        let mut x = 0;
        let mut y = 0;
        let mut d = 0;

        loop {
            if x < 0 || x >= m || y < 0 || y >= n || vis[x as usize][y as usize] {
                break;
            }
            ans.push(array[x as usize][y as usize]);
            vis[x as usize][y as usize] = true;

            // move
            let (xx, yy) = (x + dd[d].0, y + dd[d].1);
            if 0 <= xx && xx < m && 0 <= yy && yy < n && !vis[xx as usize][yy as usize] {
                (x, y) = (xx, yy);
            } else {
                d = (d + 1) % 4;
                (x, y) = (x + dd[d].0, y + dd[d].1)
            }
        }

        ans
    }
}

#[test]
fn test1() {
    let array = vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
    let ans = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(Solution::spiral_array(array), ans);
}

#[test]
fn test2() {
    let array = vec![
        vec![1, 2, 3, 4],
        vec![12, 13, 14, 5],
        vec![11, 16, 15, 6],
        vec![10, 9, 8, 7],
    ];
    let ans = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    assert_eq!(Solution::spiral_array(array), ans);
}
