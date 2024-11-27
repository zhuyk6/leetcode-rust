pub struct Solution;

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        assert!(k >= 3, "Invalid input!");
        let n = colors.len();
        let k = k as usize;

        let it0 = colors.iter().cycle();
        let mut it1 = it0.clone();
        it1.next();

        let indices = it0
            .zip(it1)
            .enumerate()
            .take(n + k - 2)
            .filter(|(_, (x, y))| x != y)
            .map(|(id, _)| id)
            .collect::<Vec<_>>();

        let mut ans = 0;
        let m = indices.len();
        let mut i = 0;
        while i < m && indices[i] < n {
            let mut j = i;
            while j < m && j - i == indices[j] - indices[i] {
                j += 1;
            }
            if j - i + 1 >= k {
                ans += j - i + 1 - k + 1;
            }
            i = j;
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let colors = vec![0, 1, 0, 1, 0];
        let k = 3;
        assert_eq!(Solution::number_of_alternating_groups(colors, k), 3);
    }

    #[test]
    fn sample2() {
        let colors = vec![0, 1, 0, 0, 1, 0, 1];
        let k = 6;
        assert_eq!(Solution::number_of_alternating_groups(colors, k), 2);
    }

    #[test]
    fn sample3() {
        let colors = vec![1, 1, 0, 1];
        let k = 4;
        assert_eq!(Solution::number_of_alternating_groups(colors, k), 0);
    }

    #[test]
    fn issue() {
        let colors = vec![0, 0, 0];
        let k = 3;
        assert_eq!(Solution::number_of_alternating_groups(colors, k), 0);
    }
}
