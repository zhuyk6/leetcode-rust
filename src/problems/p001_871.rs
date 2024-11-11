pub struct Solution;

impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        if s.as_bytes().last().unwrap() == &b'1' {
            return false;
        }

        let arr: Vec<usize> = s
            .as_bytes()
            .iter()
            .enumerate()
            .filter(|(_, &c)| c == b'0')
            .map(|(i, _)| i)
            .collect();

        let min_jump = min_jump as usize;
        let max_jump = max_jump as usize;
        let m = arr.len();

        use std::collections::VecDeque;
        let mut queue = VecDeque::with_capacity(m);
        let mut vis = vec![false; m];

        queue.push_back(0usize);
        vis[0] = true;

        let mut l = 0;
        let mut r = 0;

        while let Some(x) = queue.pop_front() {
            if x == m - 1 {
                return true;
            }
            let mut ll = l.min(x + 1);
            while ll < m && arr[ll] < arr[x] + min_jump {
                ll += 1;
            }
            let mut rr = r.max(l);
            while rr < m && arr[rr] <= arr[x] + max_jump {
                rr += 1;
            }

            #[allow(clippy::needless_range_loop)]
            for y in ll.max(r)..rr {
                if !vis[y] {
                    vis[y] = true;
                    queue.push_back(y);
                }
            }
            (l, r) = (ll, rr);
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let s = "011010".to_string();
        let min_jump = 2;
        let max_jump = 3;
        assert!(Solution::can_reach(s, min_jump, max_jump));
    }

    #[test]
    fn sample2() {
        let s = "01101110".to_string();
        let min_jump = 2;
        let max_jump = 3;
        assert!(!Solution::can_reach(s, min_jump, max_jump));
    }
}
