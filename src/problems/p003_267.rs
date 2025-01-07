use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn count_pairs(mut nums: Vec<i32>) -> i32 {
        fn to_vec(x: i32) -> Vec<i32> {
            let mut res = Vec::new();
            let mut x = x;
            while x > 0 {
                res.push(x % 10);
                x /= 10;
            }
            res.reverse();
            res
        }

        fn to_int(v: &[i32]) -> i32 {
            v.iter().fold(0, |acc, &val| acc * 10 + val)
        }

        fn exchange(x: i32) -> HashSet<i32> {
            let mut set = HashSet::new();
            let mut v = to_vec(x);
            for i in 0..v.len() {
                for j in i + 1..v.len() {
                    v.swap(i, j);
                    set.insert(to_int(&v));
                    v.swap(i, j);
                }
            }
            set
        }

        fn exchange2(x: i32) -> HashSet<i32> {
            let set1 = exchange(x);
            let mut set2 = HashSet::new();
            for &y in set1.iter() {
                set2.extend(exchange(y));
            }
            set2.insert(x);
            set2.extend(set1);
            set2
        }

        nums.sort();

        let mut arr: Vec<i32> = Vec::new();
        let mut brr: Vec<i32> = Vec::new();
        arr.push(nums[0]);
        brr.push(1);
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                arr.push(nums[i]);
                brr.push(1);
            } else {
                *brr.last_mut().unwrap() += 1;
            }
        }

        let mut ans_set = HashSet::new();
        let mut counter0: HashMap<i32, HashSet<usize>> = HashMap::new();
        let mut counter1: HashMap<i32, HashSet<usize>> = HashMap::new();
        let mut counter2: HashMap<i32, HashSet<usize>> = HashMap::new();
        for (i, &num) in arr.iter().enumerate() {
            // exchange num 0 times
            let e = counter2.entry(num).or_default();
            for &j in e.iter() {
                ans_set.insert((j, i));
            }

            // exchange num 1 times
            let set1 = exchange(num);
            for &x in set1.iter() {
                let e = counter1.entry(x).or_default();
                for &j in e.iter() {
                    ans_set.insert((j, i));
                }
            }

            // exchange num 2 times
            let set2 = exchange2(num);
            for &x in set2.iter() {
                let e = counter0.entry(x).or_default();
                for &j in e.iter() {
                    ans_set.insert((j, i));
                }
            }

            // update counter
            counter0.entry(num).or_default().insert(i);
            for &x in set1.iter() {
                counter1.entry(x).or_default().insert(i);
            }
            for &x in set2.iter() {
                counter2.entry(x).or_default().insert(i);
            }

            // dbg!(&num, &set1, &set2);
            // dbg!(&ans);
            // dbg!(&counter0);
            // dbg!(&counter1);
            // dbg!(&counter2);
        }

        let mut ans = 0;
        for (i, j) in ans_set {
            ans += brr[i] * brr[j];
        }
        for &c in brr.iter() {
            ans += c * (c - 1) / 2;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![1023, 2310, 2130, 213];
        assert_eq!(Solution::count_pairs(nums), 4);
    }

    #[test]
    fn sample2() {
        let nums = vec![1, 10, 100];
        assert_eq!(Solution::count_pairs(nums), 3);
    }

    #[test]
    fn sample3() {
        let nums = vec![1, 1, 1, 10, 10, 100];
        assert_eq!(Solution::count_pairs(nums), 15);
    }
}
