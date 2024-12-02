pub struct Solution;

impl Solution {
    pub fn non_special_count(l: i32, r: i32) -> i32 {
        fn is_prime(x: i32) -> bool {
            if x < 2 {
                false
            } else {
                let mut y = 2;
                while y * y <= x {
                    if x % y == 0 {
                        return false;
                    }
                    y += 1;
                }
                true
            }
        }

        /// check whether y * y is special
        fn is_special(y: i32) -> bool {
            y != 1 && is_prime(y)
        }

        let mut ans = 0;

        let sqrt_l = (l as f64).sqrt().floor() as i32;
        let mut y = sqrt_l;
        while y * y <= r {
            if y * y >= l && is_special(y) {
                ans += 1;
            }
            y += 1;
        }

        r - l + 1 - ans
    }
}
