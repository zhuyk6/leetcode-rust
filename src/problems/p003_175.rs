pub struct Solution;

impl Solution {
    pub fn find_winning_player<T>(skills: T, k: i32) -> i32
    where
        T: AsRef<[i32]>,
    {
        let skills = skills.as_ref();
        let k = k as usize;
        let n = skills.len();

        if k >= n - 1 {
            // select the max player
            skills
                .iter()
                .enumerate()
                .max_by_key(|&(_, &v)| v)
                .expect("skills can't be empty")
                .0 as i32
        } else {
            // simulate to find the winner
            use std::collections::VecDeque;

            let mut queue = VecDeque::<(usize, &i32)>::from_iter(skills.iter().enumerate());
            let mut num_win = 0;
            while num_win < k {
                let (id1, val1) = queue
                    .pop_front()
                    .expect("queue's len must be greater than 2");
                let (id2, val2) = queue
                    .pop_front()
                    .expect("queue's len must be greater than 2");
                if val1 > val2 {
                    num_win += 1;
                    queue.push_front((id1, val1));
                    queue.push_back((id2, val2));
                } else {
                    num_win = 1;
                    queue.push_front((id2, val2));
                    queue.push_back((id1, val1));
                }
            }
            queue
                .pop_front()
                .expect("queue's len must be greater than 2")
                .0 as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let skills = [4, 2, 6, 3, 9];
        let k = 2;
        assert_eq!(Solution::find_winning_player(skills, k), 2);
    }

    #[test]
    fn sample2() {
        let skills = [2, 5, 4];
        let k = 3;
        assert_eq!(Solution::find_winning_player(skills, k), 1);
    }
}
