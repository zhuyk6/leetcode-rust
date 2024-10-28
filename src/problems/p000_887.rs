pub struct Solution;

impl Solution {
    pub fn super_egg_drop_bisection(k: i32, n: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;

        let mut f = vec![vec![i32::MAX; k + 1]; n + 1];

        // f[0][k] = 0
        f[0].fill(0);

        for i in 1..=n {
            // f[i][0] = MAX
            f[i][0] = i32::MAX;

            // f[i][j]
            for j in 1..=k {
                // eprintln!("i = {i}, j = {j}");
                // for v in 1..=i {
                //     let tmp = f[v - 1][j - 1].max(f[i - v][j]);
                //     f[i][j] = f[i][j].min(tmp.saturating_add(1));
                //     eprintln!("v = {v}, tmp = {tmp}");
                // }

                let mut l = 1;
                let mut r = i;
                while r - l > 2 {
                    let mid = (l + r) >> 1;

                    if f[mid - 1][j - 1] < f[i - mid][j] {
                        l = mid + 1;
                    } else {
                        r = mid;
                    }
                }

                for v in l..=r {
                    let tmp = f[v - 1][j - 1].max(f[i - v][j]);
                    f[i][j] = f[i][j].min(tmp.saturating_add(1));
                }
            }
        }

        f[n][k]
    }

    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;

        let mut f = vec![vec![i32::MAX; k + 1]; n + 1];

        // f[0][k] = 0
        f[0].fill(0);

        for j in 1..=k {
            let mut opt = 1;
            for i in 1..=n {
                while opt < i
                    && f[opt - 1][j - 1].max(f[i - opt][j]) > f[opt][j - 1].max(f[i - opt - 1][j])
                {
                    opt += 1;
                }
                f[i][j] = f[opt - 1][j - 1].max(f[i - opt][j]).saturating_add(1);
            }
        }

        f[n][k]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let k = 1;
        let n = 2;
        assert_eq!(Solution::super_egg_drop(k, n), 2);
    }

    #[test]
    fn sample2() {
        let k = 2;
        let n = 6;
        assert_eq!(Solution::super_egg_drop(k, n), 3);
    }

    #[test]
    fn sample3() {
        let k = 3;
        let n = 14;
        assert_eq!(Solution::super_egg_drop(k, n), 4);
    }
}
