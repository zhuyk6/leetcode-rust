pub struct Solution;

#[derive(Debug, Clone)]
enum Expr {
    Int(i32),
    Sym(String),
    Mul(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
}

use std::collections::HashMap;

use Expr::*;

impl Expr {
    fn parse_int(s: &str) -> Option<Expr> {
        s.parse().ok().map(Int)
    }

    fn parse_sym(s: &str) -> Option<Expr> {
        Some(Sym(s.to_string()))
    }

    fn parse_expr(s: &str) -> Expr {
        // check whether op2 should push into the stack directly
        // otherwise pop the op1
        fn cmp(op1: u8, op2: u8) -> bool {
            match op2 {
                b'(' => true,
                b')' => false,
                b'+' | b'-' => op1 == b'(',
                b'*' => true,
                _ => panic!("Impossible char!"),
            }
        }

        let mut stack_op: Vec<u8> = Vec::new();
        let mut stack_expr: Vec<Expr> = Vec::new();

        let mut j = 0;
        for (i, &c) in s.as_bytes().iter().enumerate() {
            println!("i = {i}, j = {j}, c = {}", c as char);
            match c {
                b' ' => {
                    if j >= i {
                        continue;
                    }
                    let e = Expr::parse_int(&s[j..i]).or_else(|| Expr::parse_sym(&s[j..i]));
                    stack_expr.push(e.unwrap());
                    j = i + 1;
                }
                b'+' | b'-' | b'*' | b'(' => {
                    while let Some(&o) = stack_op.last() {
                        if cmp(o, c) {
                            break;
                        }
                        let e2 = stack_expr.pop().unwrap();
                        let e1 = stack_expr.pop().unwrap();
                        let _o = stack_op.pop().unwrap();
                        let e = match o {
                            b'+' => Add(Box::new(e1), Box::new(e2)),
                            b'-' => Sub(Box::new(e1), Box::new(e2)),
                            b'*' => Mul(Box::new(e1), Box::new(e2)),
                            _ => panic!("Impossible!"),
                        };
                        stack_expr.push(e);
                    }
                    stack_op.push(c);

                    j = if c == b'(' { i + 1 } else { i + 2 };
                }
                b')' => {
                    if j < i {
                        let e = Expr::parse_int(&s[j..i]).or_else(|| Expr::parse_sym(&s[j..i]));
                        stack_expr.push(e.unwrap());
                    }

                    while let Some(o) = stack_op.pop() {
                        if o == b'(' {
                            break;
                        }
                        let e2 = stack_expr.pop().unwrap();
                        let e1 = stack_expr.pop().unwrap();
                        let e = match o {
                            b'+' => Add(Box::new(e1), Box::new(e2)),
                            b'-' => Sub(Box::new(e1), Box::new(e2)),
                            b'*' => Mul(Box::new(e1), Box::new(e2)),
                            _ => panic!("Impossible!"),
                        };
                        stack_expr.push(e);
                    }
                    j = i + 2;
                }
                _ => {}
            }
        }
        // tail process
        if j < s.len() {
            let e = Expr::parse_int(&s[j..]).or(Expr::parse_sym(&s[j..]));
            stack_expr.push(e.unwrap());
        }
        while let Some(o) = stack_op.pop() {
            let e2 = stack_expr.pop().unwrap();
            let e1 = stack_expr.pop().unwrap();
            let e = match o {
                b'+' => Add(Box::new(e1), Box::new(e2)),
                b'-' => Sub(Box::new(e1), Box::new(e2)),
                b'*' => Mul(Box::new(e1), Box::new(e2)),
                _ => panic!("Impossible!"),
            };
            stack_expr.push(e);
        }

        assert!(stack_expr.len() == 1);
        assert!(stack_op.is_empty());
        stack_expr.pop().unwrap()
    }

    fn eval_expr(expr: Expr, map: &HashMap<String, i32>) -> Expr {
        match expr {
            Int(v) => Int(v),
            Sym(s) => map.get(&s).map_or(Sym(s), |v| Int(*v)),
            Add(e1, e2) => Add(
                Box::new(Expr::eval_expr(*e1, map)),
                Box::new(Expr::eval_expr(*e2, map)),
            ),
            Sub(e1, e2) => Sub(
                Box::new(Expr::eval_expr(*e1, map)),
                Box::new(Expr::eval_expr(*e2, map)),
            ),
            Mul(e1, e2) => Mul(
                Box::new(Expr::eval_expr(*e1, map)),
                Box::new(Expr::eval_expr(*e2, map)),
            ),
        }
    }

    #[allow(dead_code)]
    fn simplify(expr: Expr) -> Expr {
        fn simplify_add(x: i32, e: Expr) -> Expr {
            match e {
                Int(y) => Int(x + y),
                Add(y, z) => {
                    if let Int(y) = *y {
                        Add(Box::new(Int(x + y)), z)
                    } else {
                        Add(Box::new(Int(x)), Box::new(Add(y, z)))
                    }
                }
                Sub(y, z) => {
                    if let Int(y) = *y {
                        Sub(Box::new(Int(x + y)), z)
                    } else {
                        Add(Box::new(Int(x)), Box::new(Sub(y, z)))
                    }
                }
                _ => Add(Box::new(Int(x)), Box::new(e)),
            }
        }

        fn simplify_sub(x: i32, e: Expr) -> Expr {
            match e {
                Int(y) => Int(x - y),
                Add(y, z) => {
                    if let Int(y) = *y {
                        Sub(Box::new(Int(x - y)), z)
                    } else {
                        Sub(Box::new(Int(x)), Box::new(Add(y, z)))
                    }
                }
                Sub(y, z) => {
                    if let Int(y) = *y {
                        Add(Box::new(Int(x - y)), z)
                    } else {
                        Sub(Box::new(Int(x)), Box::new(Sub(y, z)))
                    }
                }
                _ => Sub(Box::new(Int(x)), Box::new(e)),
            }
        }

        fn mul_pushdown(e1: Expr, e2: Expr) -> Expr {
            if let Add(a, b) = e1 {
                Expr::simplify(Add(
                    Box::new(Mul(a, Box::new(e2.clone()))),
                    Box::new(Mul(b, Box::new(e2.clone()))),
                ))
            } else if let Sub(a, b) = e1 {
                Expr::simplify(Sub(
                    Box::new(Mul(a, Box::new(e2.clone()))),
                    Box::new(Mul(b, Box::new(e2.clone()))),
                ))
            } else if let Add(a, b) = e2 {
                Expr::simplify(Add(
                    Box::new(Mul(Box::new(e1.clone()), a)),
                    Box::new(Mul(Box::new(e1.clone()), b)),
                ))
            } else if let Sub(a, b) = e2 {
                Expr::simplify(Sub(
                    Box::new(Mul(Box::new(e1.clone()), a)),
                    Box::new(Mul(Box::new(e1.clone()), b)),
                ))
            } else {
                Mul(Box::new(e1), Box::new(e2))
            }
        }

        match expr {
            // Add(e1, e2) => {
            //     let a = Expr::simplify(*e1);
            //     let b = Expr::simplify(*e2);
            //     if let Int(va) = a {
            //         simplify_add(va, b)
            //     } else if let Int(vb) = b {
            //         simplify_add(vb, a)
            //     } else {
            //         Add(Box::new(a), Box::new(b))
            //     }
            // }
            // Sub(e1, e2) => {
            //     let a = Expr::simplify(*e1);
            //     let b = Expr::simplify(*e2);
            //     if let Int(va) = a {
            //         simplify_sub(va, b)
            //     } else if let Int(vb) = b {
            //         simplify_add(-vb, a)
            //     } else {
            //         Sub(Box::new(a), Box::new(b))
            //     }
            // }
            // Mul(e1, e2) => {
            //     let e1 = Expr::simplify(*e1);
            //     let e2 = Expr::simplify(*e2);
            //     if let Int(va) = e1 {
            //         if let Int(vb) = e2 {
            //             // va * vb
            //             Int(va * vb)
            //         } else {
            //             // va * e2
            //             mul_pushdown(Int(va), e2)
            //         }
            //     } else if let Int(vb) = e2 {
            //         // e1 * vb
            //         mul_pushdown(Int(vb), e1)
            //     } else {
            //         mul_pushdown(e1, e2)
            //     }
            // }
            Mul(e1, e2) => mul_pushdown(*e1, *e2),
            e => e,
        }
    }

    fn count_answer(expr: Expr) -> HashMap<String, i32> {
        match expr {
            Int(v) => HashMap::from([("".to_string(), v)]),
            Sym(s) => HashMap::from([(s, 1)]),
            Mul(e1, e2) => {
                let map1 = Expr::count_answer(*e1);
                let map2 = Expr::count_answer(*e2);
                let mut map3: HashMap<String, i32> = HashMap::new();
                for (k1, v1) in &map1 {
                    for (k2, v2) in &map2 {
                        if k1.is_empty() {
                            let s: String = k2.to_string();
                            *map3.entry(s).or_insert(0) += v1 * v2;
                        } else if k2.is_empty() {
                            let s = k1.to_string();
                            *map3.entry(s).or_insert(0) += v1 * v2;
                        } else {
                            let s1 = k1.split('*');
                            let s2 = k2.split('*');
                            let mut s = s1.chain(s2).collect::<Vec<_>>();
                            s.sort();
                            let s = s.into_iter().collect::<Vec<_>>();
                            let s = s.join("*");
                            *map3.entry(s).or_insert(0) += v1 * v2;
                        }
                    }
                }
                map3
            }
            Add(e1, e2) => {
                let map1 = Expr::count_answer(*e1);
                let map2 = Expr::count_answer(*e2);
                let mut map3: HashMap<String, i32> = HashMap::new();

                for (k1, v1) in map1 {
                    *map3.entry(k1).or_insert(0) += v1;
                }

                for (k2, v2) in map2 {
                    *map3.entry(k2).or_insert(0) += v2;
                }

                map3
            }
            Sub(e1, e2) => {
                let map1 = Expr::count_answer(*e1);
                let map2 = Expr::count_answer(*e2);
                let mut map3: HashMap<String, i32> = HashMap::new();

                for (k1, v1) in map1 {
                    *map3.entry(k1).or_insert(0) += v1;
                }

                for (k2, v2) in map2 {
                    *map3.entry(k2).or_insert(0) -= v2;
                }

                map3
            }
        }
    }
}

impl Solution {
    pub fn basic_calculator_iv(
        expression: String,
        evalvars: Vec<String>,
        evalints: Vec<i32>,
    ) -> Vec<String> {
        let expr = Expr::parse_expr(&expression);
        println!("Original: {:#?}", expr);

        let map: HashMap<String, i32> = HashMap::from_iter(evalvars.into_iter().zip(evalints));
        let expr = Expr::eval_expr(expr, &map);
        println!("Eval: {:#?}", expr);

        // let expr = Expr::simplify(expr);
        // println!("Simplified: {:#?}", expr);

        let map = Expr::count_answer(expr);
        let mut arr: Vec<(String, i32)> = map.into_iter().filter(|(_, v)| *v != 0).collect();
        arr.sort_by(|(s1, _v1), (s2, _v2)| {
            if s1.is_empty() {
                return std::cmp::Ordering::Greater;
            }
            if s2.is_empty() {
                return std::cmp::Ordering::Less;
            }
            let s1: Vec<&str> = s1.split('*').collect();
            let s2: Vec<&str> = s2.split('*').collect();
            match s1.len().cmp(&s2.len()) {
                std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
                std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
                std::cmp::Ordering::Equal => s1.join("*").cmp(&s2.join("*")),
            }
        });

        println!("Sorted: {:?}", arr);

        let mut ans = vec![];
        for (s, v) in arr {
            let mut t = String::new();
            if v < 0 {
                t.push('-');
            }
            t.push_str(&v.abs().to_string());
            if !s.is_empty() {
                t.push('*');
                t.push_str(&s);
            }
            ans.push(t);
        }
        ans
    }
}

#[test]
fn test1() {
    let expression = "e + 8 - a + 5".to_string();
    let evalvars = vec!["e".to_string()];
    let evalints = vec![1];
    assert_eq!(
        Solution::basic_calculator_iv(expression, evalvars, evalints),
        vec!["-1*a".to_string(), "14".to_string()]
    );
}

#[test]
fn test2() {
    let expression = "e - 8 + temp - press".to_string();
    let evalvars = vec!["e".to_string(), "temp".to_string()];
    let evalints = vec![1, 12];
    assert_eq!(
        Solution::basic_calculator_iv(expression, evalvars, evalints),
        vec!["-1*press".to_string(), "5".to_string()]
    );
}

#[test]
fn test3() {
    let expression = "(e + 8) * (e - 8)".to_string();
    let evalvars = vec![];
    let evalints = vec![];
    assert_eq!(
        Solution::basic_calculator_iv(expression, evalvars, evalints),
        vec!["1*e*e".to_string(), "-64".to_string()]
    );
}

#[test]
fn test4() {
    let expression = "1 + a * (b * (c + d) + e) + f".to_string();
    let evalvars = vec![];
    let evalints = vec![];
    assert_eq!(
        Solution::basic_calculator_iv(expression, evalvars, evalints),
        vec![
            "1*a*b*c".to_string(),
            "1*a*b*d".to_string(),
            "1*a*e".to_string(),
            "1*f".to_string(),
            "1".to_string()
        ]
    );
}
