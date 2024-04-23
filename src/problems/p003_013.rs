struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
        let n = nums.len();
        let dmax = dist as usize;
        let k = k as usize;

        use std::collections::BTreeSet;

        let mut s1 = BTreeSet::new();
        let mut s2 = BTreeSet::new();

        #[allow(clippy::needless_range_loop)]
        for i in 1..=(1 + dmax) {
            s2.insert((nums[i], i));
        }
        let mut sum = nums[0] as i64;
        for _ in 0..(k - 1) {
            let item = s2.pop_first().unwrap();
            sum += item.0 as i64;
            s1.insert(item);
        }
        let mut ans = sum;

        println!("s1 = {s1:?}, s2 = {s2:?}, sum = {sum}");

        for i in 2..(n - dmax) {
            // remove nums[i-1]
            if s1.contains(&(nums[i - 1], i - 1)) {
                s1.remove(&(nums[i - 1], i - 1));
                sum -= nums[i - 1] as i64;
            }
            if s2.contains(&(nums[i - 1], i - 1)) {
                s2.remove(&(nums[i - 1], i - 1));
            }

            // insert nums[i+dmax]
            s2.insert((nums[i + dmax], i + dmax));
            if s1.len() < k - 1 {
                let item = s2.pop_first().unwrap();
                sum += item.0 as i64;
                s1.insert(item);
            }
            if !s2.is_empty() && s1.last().unwrap() > s2.first().unwrap() {
                let item1 = s1.pop_last().unwrap();
                let item2 = s2.pop_first().unwrap();
                sum -= item1.0 as i64;
                sum += item2.0 as i64;
                s1.insert(item2);
                s2.insert(item1);
            }
            ans = ans.min(sum);
            println!("s1 = {s1:?}, s2 = {s2:?}, sum = {sum}");
        }

        ans
    }
}

#[test]
fn test1() {
    let nums = vec![1, 3, 2, 6, 4, 2];
    let k = 3;
    let dist = 3;
    assert_eq!(Solution::minimum_cost(nums, k, dist), 5);
}

#[test]
fn test2() {
    let nums = vec![10, 1, 2, 2, 2, 1];
    let k = 4;
    let dist = 3;
    assert_eq!(Solution::minimum_cost(nums, k, dist), 15);
}

#[test]
fn test3() {
    let nums = vec![10, 8, 18, 9];
    let k = 3;
    let dist = 1;
    assert_eq!(Solution::minimum_cost(nums, k, dist), 36);
}

#[test]
fn wrong1() {
    let nums = vec![2, 6, 3, 8, 3, 1, 1];
    let k = 3;
    let dist = 4;
    assert_eq!(Solution::minimum_cost(nums, k, dist), 4);
}
