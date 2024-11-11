pub struct Solution;

impl Solution {
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        fn dfs(i: usize, arr: &[i32], n: usize, d: usize, mem: &mut [i32]) -> i32 {
            let v = mem[i];
            if v < i32::MAX {
                return v;
            }

            let mut ret = 0i32;
            for j in i + 1..=(n - 1).min(i + d) {
                if arr[j] >= arr[i] {
                    break;
                }
                ret = ret.max(dfs(j, arr, n, d, mem));
            }
            for j in (i.saturating_sub(d)..=i.saturating_sub(1)).rev() {
                if arr[j] >= arr[i] {
                    break;
                }
                ret = ret.max(dfs(j, arr, n, d, mem))
            }
            mem[i] = ret + 1;
            ret + 1
        }

        let n = arr.len();
        let d = d as usize;
        let mut mem = vec![i32::MAX; n];

        (0..n).map(|i| dfs(i, &arr, n, d, &mut mem)).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let arr = vec![6, 4, 14, 6, 8, 13, 9, 7, 10, 6, 12];
        let d = 2;
        assert_eq!(Solution::max_jumps(arr, d), 4);
    }

    #[test]
    fn sample2() {
        let arr = vec![3, 3, 3, 3, 3];
        let d = 3;
        assert_eq!(Solution::max_jumps(arr, d), 1);
    }

    #[test]
    fn sample3() {
        let arr = vec![7, 6, 5, 4, 3, 2, 1];
        let d = 1;
        assert_eq!(Solution::max_jumps(arr, d), 7);
    }

    #[test]
    fn sample4() {
        let arr = vec![7, 1, 7, 1, 7, 1];
        let d = 2;
        assert_eq!(Solution::max_jumps(arr, d), 2);
    }

    #[test]
    fn sample5() {
        let arr = vec![66];
        let d = 1;
        assert_eq!(Solution::max_jumps(arr, d), 1);
    }
}
