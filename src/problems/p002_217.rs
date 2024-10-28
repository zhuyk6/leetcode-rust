pub struct Solution;

impl Solution {
    pub fn kth_palindrome(queries: Vec<i32>, int_length: i32) -> Vec<i64> {
        let m: i32 = (int_length + 1) / 2;

        fn get_min(m: i32) -> i64 {
            let mut x = 1;
            for _ in 1..(m as usize) {
                x *= 10;
            }
            x
        }

        fn get_max(m: i32) -> i64 {
            let mut x = 0;
            for _ in 0..(m as usize) {
                x = x * 10 + 9;
            }
            x
        }

        let min_x = get_min(m);
        let max_x = get_max(m);

        let f = |n: i32| -> i64 {
            if n as i64 > max_x - min_x + 1 {
                -1
            } else {
                min_x + n as i64 - 1
            }
        };

        let g = |x: i64| -> i64 {
            let mut acc = x;
            let mut left = if int_length % 2 == 0 { x } else { x / 10 };
            while left > 0 {
                acc = acc * 10 + left % 10;
                left /= 10;
            }
            acc
        };

        queries.into_iter().map(f).map(g).collect()
    }
}

#[test]
fn test1() {
    let queries = vec![1, 2, 3, 4, 5, 90];
    let int_length = 3;
    let answer = vec![101, 111, 121, 131, 141, 999];

    assert_eq!(Solution::kth_palindrome(queries, int_length), answer);
}

#[test]
fn test2() {
    let queries = vec![2, 4, 6];
    let int_length = 4;
    let answer = vec![1111, 1331, 1551];

    assert_eq!(Solution::kth_palindrome(queries, int_length), answer);
}
