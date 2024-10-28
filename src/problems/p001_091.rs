use std::collections::VecDeque;

pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut que: VecDeque<(usize, usize, i32)> = VecDeque::new();
    let mut vis = vec![vec![false; n]; n];

    if grid[0][0] == 1 {
        return -1;
    }
    que.push_back((0, 0, 1));
    vis[0][0] = true;

    while let Some((x, y, d)) = que.pop_front() {
        if (x, y) == (n - 1, n - 1) {
            return d;
        }
        for xx in x.saturating_sub(1)..=(x + 1).min(n - 1) {
            for yy in y.saturating_sub(1)..=(y + 1).min(n - 1) {
                if !vis[xx][yy] && grid[xx][yy] == 0 {
                    vis[xx][yy] = true;
                    que.push_back((xx, yy, d + 1));
                }
            }
        }
    }
    -1
}
