pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        heights.push(0);
        let mut ans = 0;
        let mut stk: Vec<(i32, i32)> = vec![(-1, 0)];
        for (i, h) in (0..).zip(heights) {
            while !stk.is_empty() && stk.last().unwrap().1 >= h {
                let (_, h2) = stk.pop().unwrap();
                if let Some(&(j, _)) = stk.last() {
                    ans = ans.max(h2 * (i - j - 1));
                }
            }
            stk.push((i, h));
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let heights = vec![2, 1, 5, 6, 2, 3];
        assert_eq!(Solution::largest_rectangle_area(heights), 10);
    }

    #[test]
    fn sample2() {
        let heights = vec![2, 4];
        assert_eq!(Solution::largest_rectangle_area(heights), 4);
    }
}
