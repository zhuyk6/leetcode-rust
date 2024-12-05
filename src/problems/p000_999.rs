pub struct Solution;

impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let m = board.len();
        let n = board[0].len();

        let check_pos =
            |x: i32, y: i32| -> bool { (0..m as i32).contains(&x) && (0..n as i32).contains(&y) };

        let mut s = None;
        for (i, row) in board.iter().enumerate() {
            for (j, v) in row.iter().enumerate() {
                if *v == 'R' {
                    s = Some((i, j));
                    break;
                }
            }
        }

        const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        let (x, y) = s.unwrap();
        let mut ans = 0;
        for &(dx, dy) in &DIRECTIONS {
            for t in 1.. {
                let (xx, yy) = (x as i32 + t * dx, y as i32 + t * dy);
                if check_pos(xx, yy) && board[xx as usize][yy as usize] != 'B' {
                    let (xx, yy) = (xx as usize, yy as usize);
                    if board[xx][yy] == 'p' {
                        ans += 1;
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        ans
    }
}
