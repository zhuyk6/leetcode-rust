pub struct Solution;
impl Solution {
    pub fn count_combinations(pieces: Vec<String>, positions: Vec<Vec<i32>>) -> i32 {
        #[inline]
        fn valid(x: i32, y: i32) -> bool {
            (1..=8).contains(&x) && (1..=8).contains(&y)
        }

        fn no_cover(positions: &[(i32, i32)]) -> bool {
            let n = positions.len();
            use std::collections::HashSet;
            let set: HashSet<(i32, i32)> = HashSet::from_iter(positions.iter().cloned());
            set.len() == n
        }

        fn go_move(positions: Vec<(i32, i32)>, movements: Vec<(i32, i32)>, ans: &mut i32) {
            let new_positions: Vec<(i32, i32)> = positions
                .into_iter()
                .zip(&movements)
                .map(|((x, y), (dx, dy))| (x + dx, y + dy))
                .collect();
            if new_positions.iter().all(|&(x, y)| valid(x, y)) && no_cover(&new_positions) {
                dfs(new_positions, movements, ans);
            }
        }

        fn dfs(positions: Vec<(i32, i32)>, movements: Vec<(i32, i32)>, ans: &mut i32) {
            *ans += 1;

            let n = positions.len();
            // find all movable positions
            let mut mask = 0;
            #[allow(clippy::needless_range_loop)]
            for i in 0..n {
                if movements[i] != (0, 0) {
                    mask |= 1 << i;
                }
            }

            // select all subsets of movement
            let mut s = mask;
            while s > 0 {
                let mut new_movements = movements.clone();
                #[allow(clippy::needless_range_loop)]
                for i in 0..n {
                    if s & (1 << i) == 0 {
                        new_movements[i] = (0, 0);
                    }
                }
                go_move(positions.clone(), new_movements, ans);
                s = (s - 1) & mask;
            }
        }

        fn dfs0(
            x: usize,
            pieces: &[String],
            positions: &Vec<(i32, i32)>,
            movements: &mut Vec<(i32, i32)>,
            ans: &mut i32,
        ) {
            if x >= pieces.len() {
                go_move(positions.clone(), movements.clone(), ans);
                return;
            }
            movements.push((0, 0));
            dfs0(x + 1, pieces, positions, movements, ans);
            movements.pop();

            match pieces[x].as_str() {
                "rook" => {
                    movements.push((1, 0));
                    dfs0(x + 1, pieces, positions, movements, ans);
                    movements.pop();

                    movements.push((-1, 0));
                    dfs0(x + 1, pieces, positions, movements, ans);
                    movements.pop();

                    movements.push((0, 1));
                    dfs0(x + 1, pieces, positions, movements, ans);
                    movements.pop();

                    movements.push((0, -1));
                    dfs0(x + 1, pieces, positions, movements, ans);
                    movements.pop();
                }
                "bishop" => {
                    movements.push((1, 1));
                    dfs0(x + 1, pieces, positions, movements, ans);
                    movements.pop();

                    movements.push((1, -1));
                    dfs0(x + 1, pieces, positions, movements, ans);
                    movements.pop();

                    movements.push((-1, 1));
                    dfs0(x + 1, pieces, positions, movements, ans);
                    movements.pop();

                    movements.push((-1, -1));
                    dfs0(x + 1, pieces, positions, movements, ans);
                    movements.pop();
                }
                "queen" => {
                    movements.push((1, 0));
                    dfs0(x + 1, pieces, positions, movements, ans);
                    movements.pop();

                    movements.push((-1, 0));
                    dfs0(x + 1, pieces, positions, movements, ans);
                    movements.pop();

                    movements.push((0, 1));
                    dfs0(x + 1, pieces, positions, movements, ans);
                    movements.pop();

                    movements.push((0, -1));
                    dfs0(x + 1, pieces, positions, movements, ans);
                    movements.pop();

                    movements.push((1, 1));
                    dfs0(x + 1, pieces, positions, movements, ans);
                    movements.pop();

                    movements.push((1, -1));
                    dfs0(x + 1, pieces, positions, movements, ans);
                    movements.pop();

                    movements.push((-1, 1));
                    dfs0(x + 1, pieces, positions, movements, ans);
                    movements.pop();

                    movements.push((-1, -1));
                    dfs0(x + 1, pieces, positions, movements, ans);
                    movements.pop();
                }
                _ => panic!("invalid input"),
            }
        }

        let mut ans = 0;
        let positions = positions.into_iter().map(|v| (v[0], v[1])).collect();
        let mut movements = Vec::with_capacity(pieces.len());
        dfs0(0, &pieces, &positions, &mut movements, &mut ans);

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec_owned;

    #[test]
    fn sample1() {
        let pieces = nested_vec_owned!["rook"];
        let positions = nested_vec_owned![[1, 1]];
        assert_eq!(Solution::count_combinations(pieces, positions), 15);
    }

    #[test]
    fn sample2() {
        let pieces = nested_vec_owned!["queen"];
        let positions = nested_vec_owned![[1, 1]];
        assert_eq!(Solution::count_combinations(pieces, positions), 22);
    }

    #[test]
    fn sample3() {
        let pieces = nested_vec_owned!["bishop"];
        let positions = nested_vec_owned![[4, 3]];
        assert_eq!(Solution::count_combinations(pieces, positions), 12);
    }

    #[test]
    fn sample4() {
        let pieces = nested_vec_owned!["rook", "rook"];
        let positions = nested_vec_owned![[1, 1], [8, 8]];
        assert_eq!(Solution::count_combinations(pieces, positions), 223);
    }

    #[test]
    fn sample5() {
        let pieces = nested_vec_owned!["queen", "bishop"];
        let positions = nested_vec_owned![[5, 7], [3, 4]];
        assert_eq!(Solution::count_combinations(pieces, positions), 281);
    }
}
