struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_maximum_elegance(mut items: Vec<Vec<i32>>, k: i32) -> i64 {
        let k = k as usize;
        items.sort_unstable_by_key(|element| -element[0]);

        use std::collections::HashSet;
        let mut categories = HashSet::<i32>::new();
        let mut stack = Vec::<i32>::new();
        let mut acc: i64 = 0;

        for item in &items[..k] {
            let profit = item[0];
            let category = item[1];
            acc += profit as i64;

            if categories.contains(&category) {
                stack.push(profit);
            } else {
                categories.insert(category);
            }
        }

        let mut ans = acc + (categories.len() as i64).pow(2);
        // eprintln!("ans = {ans}, stack = {stack:?}, categories: {categories:?}");

        for item in &items[k..] {
            let profit = item[0];
            let category = item[1];

            if !categories.contains(&category) && !stack.is_empty() {
                acc -= stack.pop().unwrap() as i64;
                categories.insert(category);
                acc += profit as i64;
                ans = ans.max(acc + (categories.len() as i64).pow(2));

                // eprintln!("ans = {ans}");
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
        let items = [[3, 2], [5, 1], [10, 1]];
        let k = 2;
        let items: Vec<Vec<i32>> = items.into_iter().map(Vec::from).collect();
        assert_eq!(Solution::find_maximum_elegance(items, k), 17);
    }

    #[test]
    fn sample2() {
        let items = [[3, 1], [3, 1], [2, 2], [5, 3]];
        let k = 3;
        let items: Vec<Vec<i32>> = items.into_iter().map(Vec::from).collect();
        assert_eq!(Solution::find_maximum_elegance(items, k), 19);
    }

    #[test]
    fn sample3() {
        let items = [[1, 1], [2, 1], [3, 1]];
        let k = 3;
        let items: Vec<Vec<i32>> = items.into_iter().map(Vec::from).collect();
        assert_eq!(Solution::find_maximum_elegance(items, k), 7);
    }
}
