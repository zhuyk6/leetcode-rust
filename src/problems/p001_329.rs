struct Solution;

#[allow(unused)]
impl Solution {
    pub fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();

        let mut tmp = Vec::with_capacity(m.max(n));

        let mut process = |x: usize, y: usize| {
            tmp.clear();
            for i in 0..(m.max(n)) {
                if x + i >= m || y + i >= n {
                    break;
                }
                tmp.push(mat[x + i][y + i]);
            }
            tmp.sort_unstable();
            for (i, v) in tmp.iter().enumerate() {
                mat[x + i][y + i] = *v;
            }
        };

        for i in 1..m {
            process(i, 0);
        }
        for i in 1..n {
            process(0, i);
        }
        process(0, 0);
        mat
    }
}

#[test]
fn test1() {
    let mat = [[3, 3, 1, 1], [2, 2, 1, 2], [1, 1, 1, 2]];
    let mat: Vec<Vec<i32>> = mat.into_iter().map(Vec::from).collect();
    let ans = [[1, 1, 1, 1], [1, 2, 2, 2], [1, 2, 3, 3]];
    let ans: Vec<Vec<i32>> = ans.into_iter().map(Vec::from).collect();
    assert_eq!(Solution::diagonal_sort(mat), ans);
}
