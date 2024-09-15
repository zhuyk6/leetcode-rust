struct Solution;

use std::cmp::Reverse;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
        let n = charge_times.len();

        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::<(i32, Reverse<usize>)>::new();
        let mut sum = 0i64;

        let mut ans = 0;
        let mut j = 0;
        #[allow(clippy::needless_range_loop)]
        for i in 0..n {
            if j < i {
                j = i;
                sum = 0;
            }
            while !heap.is_empty() && heap.peek().unwrap().1 .0 < i {
                heap.pop();
            }
            while j < n {
                let _sum = sum + running_costs[j] as i64;
                let c_max = if let Some(&(max, _)) = heap.peek() {
                    charge_times[j].max(max)
                } else {
                    charge_times[j]
                };
                // println!("_sum = {_sum}, cmax = {c_max}");
                if c_max as i64 + _sum * ((j - i + 1) as i64) <= budget {
                    heap.push((charge_times[j], Reverse(j)));
                    sum = _sum;
                    j += 1;
                } else {
                    break;
                }
            }
            // println!("i = {i}, j = {j}\n\n");
            ans = ans.max(j - i);
            sum -= running_costs[i] as i64;
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let charge_times = vec![3, 6, 1, 3, 4];
        let running_costs = vec![2, 1, 3, 4, 5];
        let budget = 25;
        assert_eq!(
            Solution::maximum_robots(charge_times, running_costs, budget),
            3
        );
    }

    #[test]
    fn sample2() {
        let charge_times = vec![11, 12, 19];
        let running_costs = vec![10, 8, 7];
        let budget = 19;
        assert_eq!(
            Solution::maximum_robots(charge_times, running_costs, budget),
            0
        );
    }

    #[test]
    fn sample3() {
        let charge_times = vec![11, 12, 74, 67, 37, 87, 42, 34, 18, 90, 36, 28, 34, 20];
        let running_costs = vec![18, 98, 2, 84, 7, 57, 54, 65, 59, 91, 7, 23, 94, 20];
        let budget = 937;
        assert_eq!(
            Solution::maximum_robots(charge_times, running_costs, budget),
            4
        );
    }

    #[test]
    fn sample4() {
        let charge_times = vec![
            32, 83, 96, 70, 98, 80, 30, 42, 63, 67, 49, 10, 80, 13, 69, 91, 73, 10,
        ];
        let running_costs = vec![
            49, 67, 92, 26, 18, 22, 38, 34, 36, 26, 32, 84, 39, 42, 88, 51, 8, 2,
        ];
        let budget = 599;
        assert_eq!(
            Solution::maximum_robots(charge_times, running_costs, budget),
            4
        );
    }
}
