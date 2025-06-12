pub struct Solution;

impl Solution {
    pub fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
        use std::collections::VecDeque;

        let mut que: VecDeque<(usize, i32)> = VecDeque::new();
        let mut ans = i32::MIN;

        let mut forward_iter = points.iter().enumerate().peekable();
        for (i, pi) in points.iter().enumerate() {
            while let Some(&(j, pj)) = forward_iter.peek() {
                if pj[0] - pi[0] <= k {
                    let z = pj[0] + pj[1];
                    while let Some(&(_, v)) = que.back() {
                        if v <= z {
                            que.pop_back();
                        } else {
                            break;
                        }
                    }
                    que.push_back((j, z));
                    forward_iter.next();
                } else {
                    break;
                }
            }
            while let Some(&(j, _)) = que.front() {
                if j <= i {
                    que.pop_front();
                } else {
                    break;
                }
            }
            if let Some(&(_, v)) = que.front() {
                ans = ans.max(pi[1] - pi[0] + v);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let points = nested_vec![[1, 3], [2, 0], [5, 10], [6, -10]];
        let k = 1;
        assert_eq!(Solution::find_max_value_of_equation(points, k), 4);
    }

    #[test]
    fn sample2() {
        let points = nested_vec![[0, 0], [3, 0], [9, 2]];
        let k = 3;
        assert_eq!(Solution::find_max_value_of_equation(points, k), 3);
    }
}
