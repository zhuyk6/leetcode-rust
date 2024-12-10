pub struct Solution;

impl Solution {
    pub fn kth_character(k: i64, operations: Vec<i32>) -> char {
        let mut m = 1;
        while 1 << m < k {
            m += 1;
        }
        dbg!(&m);

        fn dfs(k: i64, m: usize, ops: &[i32]) -> char {
            // dbg!(&k, &m);
            assert!(k > 0 && k <= (1 << m));
            if m == 0 {
                return 'a';
            }
            if k <= (1 << (m - 1)) {
                dfs(k, m - 1, ops)
            } else {
                let c = dfs(k - (1 << (m - 1)), m - 1, ops);
                match ops[m - 1] {
                    0 => c,
                    1 => match c {
                        'z' => 'a',
                        c => char::from_u32(c as u32 + 1).unwrap(),
                    },
                    _ => panic!("invalid input!"),
                }
            }
        }

        dfs(k, m, &operations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let k = 5;
        let operations = vec![0, 0, 0];
        assert_eq!(Solution::kth_character(k, operations), 'a')
    }

    #[test]
    fn sample2() {
        let k = 10;
        let operations = vec![0, 1, 0, 1];
        assert_eq!(Solution::kth_character(k, operations), 'b')
    }
}
