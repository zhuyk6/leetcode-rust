pub struct Solution;

use std::collections::BTreeMap;

impl Solution {
    pub fn max_task_assign(
        mut tasks: Vec<i32>,
        mut workers: Vec<i32>,
        pills: i32,
        strength: i32,
    ) -> i32 {
        tasks.sort_unstable();
        workers.sort_unstable();

        let n = tasks.len();
        let m = workers.len();
        let pills = pills as usize;

        let check = |k: usize| -> bool {
            // check whether workers[m-k..] can finish tasks[..k]
            let tasks = &tasks[..k];
            let mut workers: BTreeMap<i32, u32> =
                workers[m - k..]
                    .iter()
                    .copied()
                    .fold(BTreeMap::new(), |mut map, w| {
                        *map.entry(w).or_insert(0) += 1;
                        map
                    });

            let mut pills = pills;

            for &t in tasks.iter().rev() {
                let mut e_max = workers.last_entry().unwrap();
                if *e_max.key() >= t {
                    *e_max.get_mut() -= 1;
                    if *e_max.get() == 0 {
                        e_max.remove_entry();
                    }
                } else {
                    if pills == 0 {
                        return false;
                    }
                    pills -= 1;

                    let mut w_range = workers.range_mut(t - strength..);
                    if let Some((&w, c)) = w_range.next() {
                        *c -= 1;
                        if *c == 0 {
                            workers.remove(&w);
                        }
                    } else {
                        return false;
                    }
                }
            }

            true
        };

        let mut left = 0;
        let mut right = n.min(m);

        if !check(left) {
            return 0;
        }

        while left < right {
            let mid = (left + right).div_ceil(2);
            if check(mid) {
                left = mid;
            } else {
                right = mid - 1;
            }
        }

        left as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let tasks = vec![3, 2, 1];
        let workers = vec![0, 3, 3];
        let pills = 1;
        let strength = 1;
        assert_eq!(
            Solution::max_task_assign(tasks, workers, pills, strength),
            3
        );
    }

    #[test]
    fn sample2() {
        let tasks = vec![5, 4];
        let workers = vec![0, 0, 0];
        let pills = 1;
        let strength = 5;
        assert_eq!(
            Solution::max_task_assign(tasks, workers, pills, strength),
            1
        );
    }

    #[test]
    fn sample3() {
        let tasks = vec![10, 15, 30];
        let workers = vec![0, 10, 10, 10, 10];
        let pills = 3;
        let strength = 10;
        assert_eq!(
            Solution::max_task_assign(tasks, workers, pills, strength),
            2
        );
    }

    #[test]
    fn sample4() {
        let tasks = vec![5, 9, 8, 5, 9];
        let workers = vec![1, 6, 4, 2, 6];
        let pills = 1;
        let strength = 5;
        assert_eq!(
            Solution::max_task_assign(tasks, workers, pills, strength),
            3
        );
    }
}
