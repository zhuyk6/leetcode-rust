pub struct Solution;

impl Solution {
    pub fn find_kth_number(n: i32, mut k: i32) -> i32 {
        fn to_num(prefix: &[i32]) -> i32 {
            prefix.iter().fold(0, |acc, &d| acc * 10 + d)
        }

        fn to_digits(num: i32) -> Vec<i32> {
            let mut digits = vec![];
            let mut n = num;
            while n > 0 {
                digits.push(n % 10);
                n /= 10;
            }
            digits.reverse();
            digits
        }

        fn count(max_digits: &[i32], prefix: &[i32], digit: i32) -> i32 {
            let m = prefix.len();
            let left_len = max_digits.len() - prefix.len() - 1;

            let mut cnt = 0;
            for i in 0..left_len {
                cnt += 10i32.pow(i as u32);
            }

            let num1 = to_num(&max_digits[..m + 1]);
            let num2 = to_num(prefix) * 10 + digit;
            if num2 < num1 {
                cnt += 10i32.pow(left_len as u32);
            } else if num2 == num1 {
                cnt += to_num(&max_digits[m + 1..]) + 1;
            }

            cnt
        }

        let max_digits = to_digits(n);

        let mut prefix: Vec<i32> = vec![];

        while k > 0 {
            let start = if prefix.is_empty() { 1 } else { 0 };
            for digit in start..10 {
                let cnt = count(&max_digits, &prefix, digit);
                // println!(
                //     "k: {}, Digit: {}, Count: {}, Prefix: {:?}",
                //     k, digit, cnt, prefix
                // );
                if cnt >= k {
                    prefix.push(digit);
                    k -= 1;
                    break;
                } else {
                    k -= cnt;
                }
            }
        }

        to_num(&prefix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let n = 13;
        let k = 2;
        assert_eq!(Solution::find_kth_number(n, k), 10);
    }

    #[test]
    fn sample2() {
        let n = 1;
        let k = 1;
        assert_eq!(Solution::find_kth_number(n, k), 1);
    }

    #[test]
    fn sample3() {
        let n = 123;
        let k = 100;
        assert_eq!(Solution::find_kth_number(n, k), 78);
    }

    #[test]
    fn issue() {
        let n = 4289384;
        let k = 1922239;
        assert_eq!(Solution::find_kth_number(n, k), 2730010);
    }
}
