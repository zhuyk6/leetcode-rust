pub struct Solution;

const P: i32 = 11;

struct Dfs {
    n: usize,
    m: usize,
    needs: Vec<i32>,
    dp: Vec<i32>,
}

impl Dfs {
    fn with_needs(needs: Vec<i32>) -> Self {
        let n = needs.len();
        let m = Dfs::mask(&needs);
        let mut dp = vec![i32::MAX; m + 1];
        dp[0] = 0;
        Self { n, needs, dp, m }
    }

    #[inline]
    fn mask(nums: &[i32]) -> usize {
        let mut m = 0;
        for &v in nums {
            m = m * P + v;
        }
        m as usize
    }

    // #[inline]
    // fn less_or_eq(x: &[i32], y: &[i32]) -> bool {
    //     x.iter().zip(y).all(|(x, y)| x <= y)
    // }

    // fn unmask(&self, m: usize) -> Vec<i32> {
    //     let mut m = m as i32;
    //     let mut nums = vec![0; self.n];
    //     for i in 0..self.n {
    //         nums[self.n - 1 - i] = m % P;
    //         m /= P;
    //     }
    //     nums
    // }

    fn with_bag(&mut self, bag: &[i32], p: i32) {
        let v = self
            .needs
            .iter()
            .zip(bag)
            .map(|(&x, &y)| x - y)
            .collect::<Vec<_>>();
        if v.iter().min().unwrap() >= &0 {
            let mut acc = vec![0; self.n];
            self.dfs(0, &mut acc, &v, bag, p);
        }
    }

    fn dfs(&mut self, id: usize, acc: &mut [i32], v: &[i32], bag: &[i32], p: i32) {
        if id >= self.n {
            let i = Dfs::mask(acc);
            let k = Dfs::mask(bag);
            let j = i + k;
            self.dp[j] = self.dp[j].min(self.dp[i].saturating_add(p));
            return;
        }
        for val in 0..=v[id] {
            acc[id] = val;
            self.dfs(id + 1, acc, v, bag, p);
        }
    }
}

impl Solution {
    pub fn shopping_offers(price: Vec<i32>, mut special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        let n = price.len();

        for (id, p) in price.into_iter().enumerate() {
            let mut v = vec![0; n];
            v[id] = 1;
            v.push(p);
            special.push(v);
        }

        let mut dfs = Dfs::with_needs(needs);

        for bag in special {
            let p = bag[n];
            dfs.with_bag(&bag[..n], p);
        }

        dfs.dp[dfs.m]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let price = vec![2, 5];
        let special = nested_vec![[3, 0, 5], [1, 2, 10]];
        let needs = vec![3, 2];
        assert_eq!(Solution::shopping_offers(price, special, needs), 14);
    }

    #[test]
    fn sample2() {
        let price = vec![2, 3, 4];
        let special = nested_vec![[1, 1, 0, 4], [2, 2, 1, 9]];
        let needs = vec![1, 2, 1];
        assert_eq!(Solution::shopping_offers(price, special, needs), 11);
    }

    #[test]
    fn issue() {
        let price = vec![4, 6, 10, 2, 1, 2];
        let special = nested_vec![[3, 4, 4, 2, 5, 4, 21], [6, 4, 3, 4, 5, 2, 5]];
        let needs = vec![5, 5, 0, 3, 0, 0];
        assert_eq!(Solution::shopping_offers(price, special, needs), 56);
    }

    #[test]
    fn timeout() {
        let price = vec![7, 3, 2, 1, 4, 3];
        let special = nested_vec![
            [2, 2, 0, 3, 1, 6, 8],
            [5, 1, 6, 0, 0, 5, 13],
            [2, 3, 5, 2, 0, 3, 2],
            [3, 1, 2, 1, 5, 2, 9],
            [0, 2, 3, 2, 4, 4, 14],
            [4, 3, 1, 2, 2, 6, 3]
        ];
        let needs = vec![3, 4, 4, 3, 0, 0];
        assert_eq!(Solution::shopping_offers(price, special, needs), 44);
    }
}
