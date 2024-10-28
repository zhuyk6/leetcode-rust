#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn least_ops_express_target(x: i32, target: i32) -> i32 {
        #[inline]
        fn pow(_x: i32, k: i32) -> i32 {
            match k {
                0 => 1,
                1 => 0,
                k if k > 0 => k - 1,
                _ => panic!("k can't be negative!"),
            }
        }

        #[inline]
        fn term(a: i32, x: i32, k: i32) -> i32 {
            assert!(a > 0, "a must be positive!");
            a * pow(x, k) + (a - 1)
        }

        #[inline]
        fn plus_term(a: i32, x: i32, k: i32) -> i32 {
            match a {
                0 => 0,
                a if a > 0 => 1 + term(a, x, k),
                _ => panic!("a should be positive!"),
            }
        }

        fn factor(x: i32, mut t: i32) -> Vec<i32> {
            assert!(t > 0, "t must be positive");
            let mut v = vec![];
            while t > 0 {
                let a = t % x;
                v.push(a);
                t /= x;
            }
            v
        }

        let mut arr = factor(x, target);
        arr.push(0);
        dbg!(&arr);

        fn dfs(arr: &[i32], x: i32, k: i32, carry: bool, mem: &mut [[i32; 2]]) -> i32 {
            let p = k as usize;
            if p >= arr.len() {
                return 0;
            }

            // use memory
            let tmp = mem[p][carry as usize];
            if tmp != i32::MAX {
                return tmp;
            }

            let a = if carry { arr[p] + 1 } else { arr[p] };

            // use a
            let op1 = plus_term(a, x, k) + dfs(arr, x, k + 1, false, mem);
            // use x - a
            let op2 = plus_term(x - a, x, k) + dfs(arr, x, k + 1, true, mem);

            eprintln!("x = {x}, k = {k}, carry = {carry}, a = {a}");
            eprintln!("op1 = {op1}, op2 = {op2}");

            let tmp = op1.min(op2);
            mem[p][carry as usize] = tmp;
            tmp
        }

        let mut mem = vec![[i32::MAX; 2]; arr.len()];
        dfs(&arr, x, 0, false, &mut mem) - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let x = 3;
        let target = 19;
        assert_eq!(Solution::least_ops_express_target(x, target), 5);
    }

    #[test]
    fn sample2() {
        let x = 5;
        let target = 501;
        assert_eq!(Solution::least_ops_express_target(x, target), 8);
    }

    #[test]
    fn sample3() {
        let x = 100;
        let target = 100_000_000;
        assert_eq!(Solution::least_ops_express_target(x, target), 3);
    }
}
