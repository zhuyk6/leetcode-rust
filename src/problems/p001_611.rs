struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        // n = 0 or 1
        if n <= 1 {
            return n;
        }
        if n == 2 {
            return 3;
        } else if n == 3 {
            return 2;
        }

        // 10000.. -> 00000, total length is (m+1)
        fn f(m: i32) -> i32 {
            if m == 0 {
                1
            } else {
                1 + 2 * f(m - 1)
            }
        }

        /// flip the high bit of x
        fn flip(high: i32, x: i32) -> i32 {
            println!("h={high}, x={x}");
            if high == 0 {
                return 1;
            }
            if (x >> (high - 1)) & 1 > 0 {
                let y = if x & (1 << high) > 0 {
                    x - (1 << high) - (1 << (high - 1))
                } else {
                    x - (1 << (high - 1))
                };
                Solution::minimum_one_bit_operations(y) + 1 + f(high - 1)
            } else {
                let y = if x & (1 << high) > 0 {
                    x - (1 << high)
                } else {
                    x
                };
                flip(high - 1, y) + 1 + f(high - 1)
            }
        }

        // length of n > 2
        let mut i = 31;
        while i > 0 {
            if (n >> i) & 1 > 0 {
                break;
            }
            i -= 1;
        }
        flip(i, n)
    }
}

#[test]
fn test1() {
    let n = 3;
    assert_eq!(Solution::minimum_one_bit_operations(n), 2);
}

#[test]
fn test2() {
    let n = 6;
    assert_eq!(Solution::minimum_one_bit_operations(n), 4);
}

#[test]
fn test3() {
    let n = 102; // 1100110
    assert_eq!(Solution::minimum_one_bit_operations(n), 68);
}

#[test]
fn test4() {
    let n = 16;
    assert_eq!(Solution::minimum_one_bit_operations(n), 31);
}
