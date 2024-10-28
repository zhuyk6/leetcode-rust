pub struct Solution;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        type Bits = [i32; 32];
        // fn get_bits(v: i32) -> Bits {
        //     let mut bits = Bits::default();
        //     let mut i = 0;
        //     while 1 << i <= v {
        //         bits[i] = v & 1;
        //         i += 1;
        //     }
        //     bits
        // }

        fn from_bits(bits: Bits) -> i32 {
            bits.into_iter()
                .enumerate()
                .map(|(i, v)| if v > 0 { 1 << i } else { 0 })
                .sum()
        }

        fn add(mut acc: Bits, v: i32) -> Bits {
            let mut i = 0;
            while 1 << i <= v {
                acc[i] += (v >> i) & 1;
                i += 1;
            }
            acc
        }

        fn sub(mut acc: Bits, v: i32) -> Bits {
            let mut i = 0;
            while 1 << i <= v {
                acc[i] -= (v >> i) & 1;
                i += 1;
            }
            acc
        }

        let n = nums.len();

        let mut ans = i32::MAX;
        let mut acc = Bits::default();
        let mut r = 0;
        for l in 0..n {
            dbg!(l);
            dbg!(acc);

            r = r.max(l);
            while r < n && from_bits(add(acc, nums[r])) <= k {
                acc = add(acc, nums[r]);
                dbg!(r, acc);
                r += 1;
            }

            if l < r {
                ans = ans.min((from_bits(acc) - k).abs());
            }
            if r < n {
                ans = ans.min((from_bits(add(acc, nums[r])) - k).abs());
            }

            dbg!(ans);

            // sub nums[l]
            if l < r {
                acc = sub(acc, nums[l]);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![1, 2, 4, 5];
        let k = 3;
        assert_eq!(Solution::minimum_difference(nums, k), 0);
    }

    #[test]
    fn sample2() {
        let nums = vec![1, 3, 1, 3];
        let k = 2;
        assert_eq!(Solution::minimum_difference(nums, k), 1);
    }

    #[test]
    fn sample3() {
        let nums = vec![1];
        let k = 10;
        assert_eq!(Solution::minimum_difference(nums, k), 9);
    }

    #[test]
    fn wrong1() {
        let nums = vec![6];
        let k = 2;
        assert_eq!(Solution::minimum_difference(nums, k), 4);
    }

    #[test]
    fn wrong2() {
        let nums = vec![1, 10, 6];
        let k = 7;
        assert_eq!(Solution::minimum_difference(nums, k), 1);
    }
}
