pub struct Solution;

impl Solution {
    pub fn stone_game_ix(stones: Vec<i32>) -> bool {
        let mut cnt = [0; 3];
        for v in stones {
            cnt[(v % 3) as usize] += 1;
        }

        dbg!(&cnt);

        /// id: false Alice, true Bob
        /// pos: false state 1, true state 2
        fn check(id: bool, pos: bool, c0: u32, c1: u32, c2: u32) -> bool {
            // remove 0
            if c0 > 0 {
                if c0 % 2 == 0 {
                    check(id, pos, 0, c1, c2)
                } else {
                    !check(!id, pos, 0, c1, c2)
                }
            } else if !pos {
                // state 1
                match c1.cmp(&c2) {
                    std::cmp::Ordering::Less => false,
                    std::cmp::Ordering::Equal => id,
                    std::cmp::Ordering::Greater => {
                        if c1 == c2 + 1 {
                            id
                        } else {
                            true
                        }
                    }
                }
            } else {
                // state 2
                match c1.cmp(&c2) {
                    std::cmp::Ordering::Less => {
                        if c1 + 1 == c2 {
                            id
                        } else {
                            true
                        }
                    }
                    std::cmp::Ordering::Equal => id,
                    std::cmp::Ordering::Greater => false,
                }
            }
        }

        (cnt[1] > 0 && !check(true, false, cnt[0], cnt[1] - 1, cnt[2]))
            || (cnt[2] > 0 && !check(true, true, cnt[0], cnt[1], cnt[2] - 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let stones = vec![2, 1];
        assert!(Solution::stone_game_ix(stones));
    }

    #[test]
    fn sample2() {
        let stones = vec![2];
        assert!(!Solution::stone_game_ix(stones));
    }

    #[test]
    fn sample3() {
        let stones = vec![5, 1, 2, 4, 3];
        assert!(!Solution::stone_game_ix(stones));
    }

    #[test]
    fn issue1() {
        let stones = vec![20, 3, 20, 17, 2, 12, 15, 17, 4];
        assert!(Solution::stone_game_ix(stones));
    }

    #[test]
    fn issue2() {
        let stones = vec![2, 2, 3];
        assert!(!Solution::stone_game_ix(stones));
    }
}
