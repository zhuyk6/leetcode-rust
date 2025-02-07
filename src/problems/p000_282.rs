pub struct Solution;

#[derive(Debug, Clone, Copy)]
enum ExprSym {
    Num(i64),
    Add,
    Sub,
    Mul,
}

impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let n = num.len();
        let s = num.as_str();

        let get_symbols = |mask: u32| -> Option<Vec<ExprSym>> {
            // get symbols
            let mut symbols: Vec<ExprSym> = vec![];
            let mut last: (usize, usize) = (0, 1);
            for bit in 0..(n - 1) {
                match (mask >> (2 * bit)) & 3 {
                    0 => {
                        // no op
                        last.1 += 1;
                    }
                    1 => {
                        // op is +
                        if last.1 > last.0 + 1 && &s[last.0..last.0 + 1] == "0" {
                            return None;
                        }
                        symbols.push(ExprSym::Num(s[last.0..last.1].parse().unwrap()));
                        symbols.push(ExprSym::Add);
                        last = (bit + 1, bit + 2);
                    }
                    2 => {
                        // op is -
                        if last.1 > last.0 + 1 && &s[last.0..last.0 + 1] == "0" {
                            return None;
                        }
                        symbols.push(ExprSym::Num(s[last.0..last.1].parse().unwrap()));
                        symbols.push(ExprSym::Sub);
                        last = (bit + 1, bit + 2);
                    }
                    _ => {
                        // op is *
                        if last.1 > last.0 + 1 && &s[last.0..last.0 + 1] == "0" {
                            return None;
                        }
                        symbols.push(ExprSym::Num(s[last.0..last.1].parse().unwrap()));
                        symbols.push(ExprSym::Mul);
                        last = (bit + 1, bit + 2);
                    }
                }
            }
            if last.0 < n {
                if last.1 > last.0 + 1 && &s[last.0..last.0 + 1] == "0" {
                    return None;
                }
                symbols.push(ExprSym::Num(s[last.0..last.1].parse().unwrap()));
            }

            Some(symbols)
        };

        let calc = |symbols: &[ExprSym]| -> i64 {
            let mut stk_op: Vec<ExprSym> = vec![];
            let mut stk: Vec<i64> = vec![];
            for sym in symbols {
                match sym {
                    ExprSym::Num(v) => stk.push(*v),
                    ExprSym::Add | ExprSym::Sub => {
                        while let Some(op) = stk_op.pop() {
                            let b = stk.pop().unwrap();
                            let a = stk.pop().unwrap();
                            match op {
                                ExprSym::Add => stk.push(a + b),
                                ExprSym::Sub => stk.push(a - b),
                                ExprSym::Mul => stk.push(a * b),
                                _ => unreachable!(),
                            }
                        }
                        stk_op.push(*sym)
                    }
                    ExprSym::Mul => stk_op.push(*sym),
                }
            }

            while let Some(op) = stk_op.pop() {
                let b = stk.pop().unwrap();
                let a = stk.pop().unwrap();
                match op {
                    ExprSym::Add => stk.push(a + b),
                    ExprSym::Sub => stk.push(a - b),
                    ExprSym::Mul => stk.push(a * b),
                    _ => unreachable!(),
                }
            }
            assert!(stk.len() == 1);
            stk.pop().unwrap()
        };

        fn get_ans(symbols: &[ExprSym]) -> String {
            let mut ans = String::new();
            for sym in symbols {
                match sym {
                    ExprSym::Num(v) => ans.push_str(&v.to_string()),
                    ExprSym::Add => ans.push('+'),
                    ExprSym::Sub => ans.push('-'),
                    ExprSym::Mul => ans.push('*'),
                }
            }
            ans
        }

        let mut ans: Vec<String> = vec![];
        for mask in 0u32..(1 << (2 * (n - 1))) {
            if let Some(symbols) = get_symbols(mask) {
                // dbg!(&symbols);
                let v = calc(&symbols);
                if v == target as i64 {
                    ans.push(get_ans(&symbols));
                }
            }
        }

        // dbg!(&ans);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let num = "123".to_string();
        let target = 6;
        let ans = vec!["1+2+3".to_string(), "1*2*3".to_string()];
        assert_eq!(Solution::add_operators(num, target), ans);
    }

    #[test]
    fn sample2() {
        let num = "232".to_string();
        let target = 8;
        let ans = vec!["2*3+2".to_string(), "2+3*2".to_string()];
        assert_eq!(Solution::add_operators(num, target), ans);
    }

    #[test]
    fn sample3() {
        let num = "3456237490".to_string();
        let target = 9191;
        let ans: Vec<String> = vec![];
        assert_eq!(Solution::add_operators(num, target), ans);
    }

    #[test]
    fn sample4() {
        let num = "0123".to_string();
        let target = 6;
        let ans: Vec<String> = vec![];
        assert_eq!(Solution::add_operators(num, target), ans);
    }
}
