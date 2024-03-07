struct Solution;

#[allow(unused)]
#[inline]
fn mul(a: i32, b: i32, m: i32) -> i32 {
    (a as i64 * b as i64 % m as i64) as i32
}

#[allow(unused)]
impl Solution {
    pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
        let mut z = 0;
        let mut ans = vec![];
        for y in word.as_bytes().iter() {
            let y = (*y - b'0') as i32;
            z = (mul(10 % m, z, m) + y % m) % m;
            ans.push(if z == 0 { 1 } else { 0 });
        }
        ans
    }
}

#[test]
fn test1() {
    let word = "998244353".to_string();
    let m = 3;
    assert_eq!(
        Solution::divisibility_array(word, m),
        vec![1, 1, 0, 0, 0, 1, 1, 0, 0]
    );
}

#[test]
fn test2() {
    let word = "1010".to_string();
    let m = 10;
    assert_eq!(Solution::divisibility_array(word, m), vec![0, 1, 0, 1]);
}
