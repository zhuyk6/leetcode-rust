pub struct Solution;

impl Solution {
    pub fn maximum_beauty(
        flowers: Vec<i32>,
        new_flowers: i64,
        target: i32,
        full: i32,
        partial: i32,
    ) -> i64 {
        let mut arr: Vec<i32> = flowers.iter().filter(|&&f| f < target).copied().collect();
        let ans1 = (flowers.len() - arr.len()) as i64 * full as i64;
        if arr.is_empty() {
            return ans1;
        }

        arr.sort_unstable();

        let prefix_sum: Vec<i64> = arr
            .iter()
            .scan(0, |acc, &x| {
                *acc += x as i64;
                Some(*acc)
            })
            .collect();

        let sum = |l: usize, r: usize| -> i64 {
            assert!(l <= r);
            if l == 0 {
                prefix_sum[r]
            } else {
                prefix_sum[r] - prefix_sum[l - 1]
            }
        };

        let n = arr.len();
        let mut ans2: i64 = 0;
        let mut i: usize = 0;
        let mut j: usize = 0;

        if new_flowers >= target as i64 * n as i64 - sum(0, n - 1) {
            ans2 = ans2.max(n as i64 * full as i64);
        }

        for min in arr[0]..target {
            // make sure arr[..=i] <= min and arr[i+1..] > min
            while i + 1 < n && arr[i + 1] <= min {
                i += 1;
            }
            // add arr[..=i] to min
            let needed_flowers = min as i64 * (i + 1) as i64 - sum(0, i);
            let left_flowers = new_flowers - needed_flowers;
            if left_flowers < 0 {
                break;
            }

            // add arr[j..] to target
            while j < n && left_flowers < target as i64 * (n - j) as i64 - sum(j, n - 1) {
                j += 1;
            }

            // add arr[i+1..] to target
            let needed_flowers2 = if i + 1 == n {
                0
            } else {
                target as i64 * (n - 1 - i) as i64 - sum(i + 1, n - 1)
            };

            let tmp = if left_flowers < needed_flowers2 {
                min as i64 * partial as i64 + (n - j) as i64 * full as i64
            } else {
                // add arr[i+1..] to target and still have flowers
                let left_flowers2 = left_flowers - needed_flowers2;

                // add min to target if possible
                let num: i64 = left_flowers2 / (target - min) as i64;
                // you can't add all arr[..=i] to target
                let num_full: i64 = (n - 1 - i) as i64 + num.min(i as i64);

                min as i64 * partial as i64 + num_full * full as i64
            };
            ans2 = ans2.max(tmp);
        }

        ans1 + ans2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let flowers = vec![1, 3, 1, 1];
        let new_flowers = 7;
        let target = 6;
        let full = 12;
        let partial = 1;
        assert_eq!(
            Solution::maximum_beauty(flowers, new_flowers, target, full, partial),
            14
        );
    }

    #[test]
    fn sample2() {
        let flowers = vec![2, 4, 5, 3];
        let new_flowers = 10;
        let target = 5;
        let full = 2;
        let partial = 6;
        assert_eq!(
            Solution::maximum_beauty(flowers, new_flowers, target, full, partial),
            30
        );
    }

    #[test]
    fn issue() {
        let flowers = vec![18, 16, 10, 10, 5];
        let new_flowers = 10;
        let target = 3;
        let full = 15;
        let partial = 4;
        assert_eq!(
            Solution::maximum_beauty(flowers, new_flowers, target, full, partial),
            75
        );
    }

    #[test]
    fn issue2() {
        let flowers = vec![5, 19, 1, 1, 6, 10, 18, 12, 20, 10, 11];
        let new_flowers = 6;
        let target = 20;
        let full = 3;
        let partial = 11;
        assert_eq!(
            Solution::maximum_beauty(flowers, new_flowers, target, full, partial),
            47
        );
    }
}
