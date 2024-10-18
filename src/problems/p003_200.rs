struct Solution;

#[allow(unused)]
impl Solution {
    pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
        fn f(n: i32) -> i32 {
            (n as f64).sqrt().floor() as i32
        }

        fn g(n: i32) -> i32 {
            let mut m = f(n) - 1;
            while (m + 1) * (m + 2) <= n {
                m += 1;
            }
            m
        }

        let mut ans = 0;

        // red first
        let n = f(red);
        let m = g(blue);
        // dbg!(n, m);
        if n <= m {
            ans = ans.max(n * 2);
        } else {
            ans = ans.max(m * 2 + 1);
        }

        // blue first
        let n = f(blue);
        let m = g(red);
        // dbg!(n, m);
        if n <= m {
            ans = ans.max(n * 2);
        } else {
            ans = ans.max(m * 2 + 1);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let red = 2;
        let blue = 4;
        assert_eq!(Solution::max_height_of_triangle(red, blue), 3);
    }

    #[test]
    fn sample2() {
        let red = 2;
        let blue = 1;
        assert_eq!(Solution::max_height_of_triangle(red, blue), 2);
    }

    #[test]
    fn sample3() {
        let red = 1;
        let blue = 1;
        assert_eq!(Solution::max_height_of_triangle(red, blue), 1);
    }

    #[test]
    fn sample4() {
        let red = 10;
        let blue = 1;
        assert_eq!(Solution::max_height_of_triangle(red, blue), 2);
    }
}
