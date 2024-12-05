pub struct Solution;

const BISHOP_MOVEMENTS: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
const TROOP_MOVEMENTS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl Solution {
    pub fn min_moves_to_capture_the_queen(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
        #[inline]
        fn valid_pos(x: i32, y: i32) -> bool {
            (1..=8).contains(&x) && (1..=8).contains(&y)
        }

        #[inline]
        fn g(x: i32) -> usize {
            (x - 1) as usize
        }

        type Pos = (i32, i32);

        use std::collections::VecDeque;
        let mut que: VecDeque<(Pos, Pos, i32)> = VecDeque::new();

        let mut vis = [[[[false; 8]; 8]; 8]; 8];

        que.push_back(((a, b), (c, d), 0));
        vis[g(a)][g(b)][g(c)][g(d)] = true;

        while let Some(((a, b), (c, d), steps)) = que.pop_front() {
            if (a, b) == (e, f) || (c, d) == (e, f) {
                return steps;
            }
            // move trook
            for &(dx, dy) in &TROOP_MOVEMENTS {
                for t in 0.. {
                    let (x, y) = (a + t * dx, b + t * dy);
                    if !valid_pos(x, y) || (x, y) == (c, d) {
                        break;
                    }
                    if !vis[g(x)][g(y)][g(c)][g(d)] {
                        vis[g(x)][g(y)][g(c)][g(d)] = true;
                        que.push_back(((x, y), (c, d), steps + 1));
                    }
                }
            }

            // move bishop
            for &(dx, dy) in &BISHOP_MOVEMENTS {
                for t in 0.. {
                    let (x, y) = (c + t * dx, d + t * dy);
                    if !valid_pos(x, y) || (x, y) == (a, b) {
                        break;
                    }
                    if !vis[g(a)][g(b)][g(x)][g(y)] {
                        vis[g(a)][g(b)][g(x)][g(y)] = true;
                        que.push_back(((a, b), (x, y), steps + 1));
                    }
                }
            }
        }

        -1
    }
}
