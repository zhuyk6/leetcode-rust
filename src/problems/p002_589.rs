pub struct Solution;

impl Solution {
    pub fn find_minimum_time(mut tasks: Vec<Vec<i32>>) -> i32 {
        tasks.sort_by_key(|w| w[1]);

        let mut time = [false; 2001];

        for task in tasks {
            let s = task[0] as usize;
            let t = task[1] as usize;
            let d = task[2] as usize;

            let cnt = time[s..=t].iter().filter(|v| **v).count();
            if d > cnt {
                let mut left = d - cnt;
                for i in (s..=t).rev() {
                    if !time[i] {
                        time[i] = true;
                        left -= 1;
                        if left == 0 {
                            break;
                        }
                    }
                }
            }
        }

        time.iter().filter(|v| **v).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn sample1() {
        let tasks = array![[2, 3, 1], [4, 5, 1], [1, 5, 2]];
        let tasks: Vec<Vec<i32>> = tasks.outer_iter().map(|v| v.to_vec()).collect();

        assert_eq!(Solution::find_minimum_time(tasks), 2);
    }

    #[test]
    fn sample2() {
        let tasks = array![[1, 3, 2], [2, 5, 3], [5, 6, 2]];
        let tasks: Vec<Vec<i32>> = tasks.outer_iter().map(|v| v.to_vec()).collect();

        assert_eq!(Solution::find_minimum_time(tasks), 4);
    }

    #[test]
    fn wrong1() {
        let tasks = array![[8, 19, 1], [3, 20, 1], [1, 20, 2], [6, 13, 3]];
        let tasks: Vec<Vec<i32>> = tasks.outer_iter().map(|v| v.to_vec()).collect();

        assert_eq!(Solution::find_minimum_time(tasks), 3);
    }
}
