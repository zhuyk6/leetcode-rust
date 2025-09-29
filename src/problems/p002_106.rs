pub struct Solution;

impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        let n = fruits.len();

        let (positions, counts) = {
            let mut positions = Vec::new();
            let mut counts = Vec::new();
            for w in fruits {
                positions.push(w[0]);
                counts.push(w[1]);
            }
            (positions, counts)
        };

        let prefix_sum = {
            let mut ps = vec![0; n];
            ps[0] = counts[0];
            for i in 1..n {
                ps[i] = ps[i - 1] + counts[i];
            }
            ps
        };
        let sum = |l: usize, r: usize| {
            assert!(l <= r && r < n);

            if l == 0 {
                prefix_sum[r]
            } else {
                prefix_sum[r] - prefix_sum[l - 1]
            }
        };

        let mut ans = 0;
        let p = positions.partition_point(|&v| v <= start_pos);

        // left -> right
        let mut i = 0;
        while i < n && positions[i] < start_pos - k {
            i += 1;
        }
        let mut j = i;
        while i < n && positions[i] <= start_pos {
            j = j.max(i);
            while j < n && start_pos - positions[i] + positions[j] - positions[i] <= k {
                j += 1;
            }
            println!("i = {i}, j = {j}");
            if j < p {
                ans = ans.max(sum(i, p - 1));
            } else {
                ans = ans.max(sum(i, j - 1));
            }
            i += 1;
        }

        // right -> left
        let p = positions.partition_point(|&v| v < start_pos);
        let mut i = n - 1;
        while positions[i] > start_pos + k {
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }
        if positions[i] > start_pos + k {
            return 0;
        }
        let mut j = i;
        while positions[i] >= start_pos {
            j = j.min(i);
            while positions[i] - start_pos + positions[i] - positions[j] <= k {
                if j == 0 {
                    break;
                } else {
                    j -= 1;
                }
            }
            let t = if positions[i] - start_pos + positions[i] - positions[j] > k {
                j + 1
            } else {
                j
            };
            println!("i = {i}, t = {t}");
            if t >= p {
                ans = ans.max(sum(p, i));
            } else {
                ans = ans.max(sum(t, i));
            }

            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let fruits = nested_vec![[2, 8], [6, 3], [8, 6]];
        let start_pos = 5;
        let k = 4;
        assert_eq!(Solution::max_total_fruits(fruits, start_pos, k), 9);
    }

    #[test]
    fn sample2() {
        let fruits = nested_vec![[0, 9], [4, 1], [5, 7], [6, 2], [7, 4], [10, 9]];
        let start_pos = 5;
        let k = 4;
        assert_eq!(Solution::max_total_fruits(fruits, start_pos, k), 14);
    }

    #[test]
    fn sample3() {
        let fruits = nested_vec![[0, 3], [6, 4], [8, 5]];
        let start_pos = 3;
        let k = 2;
        assert_eq!(Solution::max_total_fruits(fruits, start_pos, k), 0);
    }
}
