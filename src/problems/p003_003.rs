pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn max_partitions_after_operations(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let k = k as usize;

        // prefix_part[i] : number of parts s[..=i] can be parted.
        let prefix_part: Vec<u32> = {
            let mut prefix_part: Vec<u32> = vec![0; n];
            let mut set: HashSet<u8> = HashSet::with_capacity(k);

            prefix_part[0] = 1;
            set.insert(s[0]);

            for (i, &ch) in s.iter().enumerate().skip(1) {
                if !set.contains(&ch) {
                    if set.len() == k {
                        set.clear();
                        set.insert(ch);
                        prefix_part[i] = prefix_part[i - 1] + 1;
                    } else {
                        set.insert(ch);
                        prefix_part[i] = prefix_part[i - 1];
                    }
                } else {
                    prefix_part[i] = prefix_part[i - 1];
                }
            }
            prefix_part
        };

        // suffix_part[i]: number of partes s[i..] can be parted
        let suffix_part: Vec<u32> = {
            let mut suffix_part: Vec<u32> = vec![0; n];
            let mut set: HashSet<u8> = HashSet::with_capacity(k);

            suffix_part[n - 1] = 1;
            set.insert(s[n - 1]);

            for (i, &ch) in s.iter().enumerate().rev().skip(1) {
                if !set.contains(&ch) {
                    if set.len() == k {
                        set.clear();
                        set.insert(ch);
                        suffix_part[i] = suffix_part[i + 1] + 1;
                    } else {
                        set.insert(ch);
                        suffix_part[i] = suffix_part[i + 1];
                    }
                } else {
                    suffix_part[i] = suffix_part[i + 1];
                }
            }
            suffix_part
        };

        let prefix_sum: Vec<[u32; 26]> = {
            let mut prefix_sum = vec![[0; 26]; n];
            prefix_sum[0][(s[0] - b'a') as usize] = 1;

            for (i, ch) in s.iter().enumerate().skip(1) {
                let ch = (ch - b'a') as usize;
                prefix_sum[i] = prefix_sum[i - 1];
                prefix_sum[i][ch] += 1;
            }

            prefix_sum
        };

        let mut ans = 0;
        let indices: Vec<usize> = (0..n).collect();

        fn count_diff(cnt: [u32; 26]) -> usize {
            let mut cnt_diff = 0;
            for v in cnt {
                if v > 0 {
                    cnt_diff += 1;
                }
            }
            cnt_diff
        }

        fn sub(c1: [u32; 26], c2: [u32; 26]) -> [u32; 26] {
            let mut res = [0; 26];
            for i in 0..26 {
                res[i] = c1[i] - c2[i];
            }
            res
        }

        for i in 0..n {
            // change `s[i]` to `ch`
            for ch in b'a'..=b'z' {
                let old_ch = (s[i] - b'a') as usize;
                let new_ch = (ch - b'a') as usize;

                let mut num_parts = 0;

                // find the previous group
                let mut left =
                    indices.partition_point(|&idx| prefix_part[idx] + 1 < prefix_part[i]);
                let mut counter1 = [0; 26];
                if left > 0 {
                    num_parts += prefix_part[left - 1];
                    counter1 = prefix_sum[left - 1];
                }

                loop {
                    let right = left
                        + indices[left..].partition_point(|&idx| {
                            let mut counter = prefix_sum[idx];
                            if idx >= i {
                                counter[old_ch] -= 1;
                                counter[new_ch] += 1;
                            }

                            count_diff(sub(counter, counter1)) <= k
                        });

                    num_parts += 1;
                    counter1 = prefix_sum[right - 1];
                    left = right;

                    if right > i {
                        if right < n {
                            num_parts += suffix_part[right];
                        }
                        break;
                    }
                }

                ans = ans.max(num_parts);
            }
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let s = "accca".to_string();
        let k = 2;
        assert_eq!(Solution::max_partitions_after_operations(s, k), 3);
    }

    #[test]
    fn sample2() {
        let s = "aabaab".to_string();
        let k = 3;
        assert_eq!(Solution::max_partitions_after_operations(s, k), 1);
    }

    #[test]
    fn sample3() {
        let s = "xxyz".to_string();
        let k = 1;
        assert_eq!(Solution::max_partitions_after_operations(s, k), 4);
    }

    #[test]
    fn issue() {
        let s = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxzabcdefghijklmnopqrstuvwxy"
            .to_string();
        let k = 25;
        assert_eq!(Solution::max_partitions_after_operations(s, k), 3);
    }
}
