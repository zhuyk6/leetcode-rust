pub struct Solution;

impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let s = s.as_bytes();
        let n = s.len();

        let mut l_most = vec![0usize; n];
        let mut nums = vec![0i64; n];
        let mut cnt = [0; 2];
        let mut j = 0;
        for i in 0..n {
            let c = (s[i] - b'0') as usize;
            cnt[c] += 1;
            while cnt[0] > k && cnt[1] > k {
                let c = (s[j] - b'0') as usize;
                cnt[c] -= 1;
                j += 1;
            }
            l_most[i] = j;
            nums[i] = (i - j + 1) as i64;
        }

        let mut pre_nums = vec![0i64; n];
        pre_nums[0] = nums[0];
        for i in 1..n {
            pre_nums[i] = pre_nums[i - 1] + nums[i];
        }

        let mut answers = Vec::with_capacity(queries.len());
        for query in queries {
            let (l, r) = (query[0] as usize, query[1] as usize);
            if l >= l_most[r] {
                let tmp = (r - l + 1) as i64;
                answers.push(tmp * (tmp + 1) / 2);
            } else {
                let rr = l_most.partition_point(|lmost| *lmost < l);
                let mut ans = 0i64;
                // l..rr
                let tmp = (rr - l) as i64;
                ans += (tmp + 1) * tmp / 2;
                // rr..=r
                ans += pre_nums[r] - rr.checked_sub(1).map(|x| pre_nums[x]).unwrap_or_default();

                answers.push(ans);
            }
        }

        answers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let s = "0001111".to_string();
        let k = 2;
        let queries = nested_vec![[0, 6]];
        assert_eq!(
            Solution::count_k_constraint_substrings(s, k, queries),
            vec![26]
        );
    }

    #[test]
    fn sample2() {
        let s = "010101".to_string();
        let k = 1;
        let queries = nested_vec![[0, 5], [1, 4], [2, 3]];
        assert_eq!(
            Solution::count_k_constraint_substrings(s, k, queries),
            vec![15, 9, 3]
        );
    }
}
