struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let n = code.len();
        let sum = |x: usize| -> i32 {
            match k.cmp(&0) {
                std::cmp::Ordering::Less => {
                    let mut s = 0;
                    for i in 1..k.unsigned_abs() as usize {
                        let y = (x + n - i) % n;
                        s += code[y];
                    }
                    s
                }
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => {
                    let mut s = 0;
                    for i in 1..k as usize {
                        let y = (x + i) % n;
                        s += code[y];
                    }
                    s
                }
            }
        };
        (0..n).map(sum).collect()
    }
}
