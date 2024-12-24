pub struct Solution;

impl Solution {
    pub fn largest_multiple_of_three(digits: Vec<i32>) -> String {
        let s = digits.iter().sum::<i32>();

        let mut cnt = [0u32; 10];
        for d in &digits {
            cnt[*d as usize] += 1;
        }

        fn remove(cnt: &mut [u32; 10], m: usize, mut num: u32) -> bool {
            if cnt[m] + cnt[m + 3] + cnt[m + 6] >= num {
                while num > 0 {
                    if cnt[m] > 0 {
                        cnt[m] -= 1;
                    } else if cnt[m + 3] > 0 {
                        cnt[m + 3] -= 1;
                    } else {
                        cnt[m + 6] -= 1;
                    }
                    num -= 1;
                }
                true
            } else {
                false
            }
        }

        #[allow(clippy::collapsible_if)]
        if s % 3 == 1 {
            if !remove(&mut cnt, 1, 1) {
                remove(&mut cnt, 2, 2);
            }
        } else if s % 3 == 2 {
            if !remove(&mut cnt, 2, 1) {
                remove(&mut cnt, 1, 2);
            }
        }

        let mut v = vec![];
        for i in (0..10).rev() {
            for _ in 0..cnt[i] {
                v.push(i as u8 + b'0');
            }
        }

        if v.is_empty() {
            "".to_string()
        } else if v[0] == b'0' {
            "0".to_string()
        } else {
            String::from_utf8(v).unwrap()
        }
    }
}
