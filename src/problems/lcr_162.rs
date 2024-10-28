pub struct Solution;

impl Solution {
    pub fn digit_one_in_number(num: i32) -> i32 {
        let mut ans = 0;
        let mut digit = 1;
        while digit <= num {
            let high = num / digit / 10;
            let low = num % digit;
            println!("digit = {digit}, high = {high}, low = {low}");
            ans += high * digit;
            match (num / digit) % 10 {
                0 => (),
                1 => ans += low + 1,
                _ => ans += digit,
            }
            if let Some(tmp) = digit.checked_mul(10) {
                digit = tmp;
            } else {
                break;
            }
        }
        ans
    }
}

#[test]
fn test1() {
    let num = 0;
    assert_eq!(Solution::digit_one_in_number(num), 0);
}

#[test]
fn test2() {
    let num = 13;
    assert_eq!(Solution::digit_one_in_number(num), 6);
}

#[test]
fn test3() {
    let num = 10;
    assert_eq!(Solution::digit_one_in_number(num), 2);
}

#[test]
fn test4() {
    let num = 1410065408;
    assert_eq!(Solution::digit_one_in_number(num), 1737167499);
}
