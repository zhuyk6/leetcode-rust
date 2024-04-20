struct Solution;

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Grid {
    lamps: HashSet<(usize, usize)>,
    rows: HashMap<usize, u32>,
    cols: HashMap<usize, u32>,
    diag1: HashMap<i32, u32>,
    diag2: HashMap<i32, u32>,
}

impl Grid {
    fn new(lamps: Vec<Vec<i32>>) -> Self {
        let lamps = HashSet::from_iter(lamps.into_iter().map(|w| (w[0] as usize, w[1] as usize)));
        let mut rows = HashMap::new();
        let mut cols = HashMap::new();
        let mut diag1 = HashMap::new();
        let mut diag2 = HashMap::new();

        for &(x, y) in &lamps {
            *rows.entry(x).or_default() += 1;
            *cols.entry(y).or_default() += 1;
            *diag1.entry(x as i32 - y as i32).or_default() += 1;
            *diag2.entry(x as i32 + y as i32).or_default() += 1;
        }

        Grid {
            lamps,
            rows,
            cols,
            diag1,
            diag2,
        }
    }

    fn query(&self, x: usize, y: usize) -> bool {
        self.rows.contains_key(&x)
            || self.cols.contains_key(&y)
            || self.diag1.contains_key(&(x as i32 - y as i32))
            || self.diag2.contains_key(&(x as i32 + y as i32))
    }

    fn delete(&mut self, x: usize, y: usize) -> bool {
        if self.lamps.contains(&(x, y)) {
            self.lamps.remove(&(x, y));

            if *self.rows.entry(x).and_modify(|c| *c -= 1).or_default() == 0 {
                self.rows.remove(&x);
            }
            if *self.cols.entry(y).and_modify(|c| *c -= 1).or_default() == 0 {
                self.cols.remove(&y);
            }
            if *self
                .diag1
                .entry(x as i32 - y as i32)
                .and_modify(|c| *c -= 1)
                .or_default()
                == 0
            {
                self.diag1.remove(&(x as i32 - y as i32));
            }
            if *self
                .diag2
                .entry(x as i32 + y as i32)
                .and_modify(|c| *c -= 1)
                .or_default()
                == 0
            {
                self.diag2.remove(&(x as i32 + y as i32));
            }
            true
        } else {
            false
        }
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut grid = Grid::new(lamps);

        queries
            .into_iter()
            .map(move |w| {
                let x = w[0] as usize;
                let y = w[1] as usize;
                let ans = grid.query(x, y);

                for xx in x.saturating_sub(1)..=(x + 1).min(n) {
                    for yy in y.saturating_sub(1)..=(y + 1).min(n) {
                        grid.delete(xx, yy);
                    }
                }

                println!("{:?}", grid);

                match ans {
                    true => 1,
                    false => 0,
                }
            })
            .collect()
    }
}

#[test]
fn test1() {
    let n = 5;
    let lamps = [[0, 0], [4, 4]];
    let queries = [[1, 1], [1, 0]];
    let lamps: Vec<Vec<i32>> = lamps.into_iter().map(Vec::from).collect();
    let queries: Vec<Vec<i32>> = queries.into_iter().map(Vec::from).collect();
    assert_eq!(Solution::grid_illumination(n, lamps, queries), vec![1, 0]);
}

#[test]
fn test2() {
    let n = 5;
    let lamps = [[0, 0], [4, 4]];
    let queries = [[1, 1], [1, 1]];
    let lamps: Vec<Vec<i32>> = lamps.into_iter().map(Vec::from).collect();
    let queries: Vec<Vec<i32>> = queries.into_iter().map(Vec::from).collect();
    assert_eq!(Solution::grid_illumination(n, lamps, queries), vec![1, 1]);
}

#[test]
fn test3() {
    let n = 5;
    let lamps = [[0, 0], [0, 4]];
    let queries = [[0, 4], [0, 1], [1, 4]];
    let lamps: Vec<Vec<i32>> = lamps.into_iter().map(Vec::from).collect();
    let queries: Vec<Vec<i32>> = queries.into_iter().map(Vec::from).collect();
    assert_eq!(
        Solution::grid_illumination(n, lamps, queries),
        vec![1, 1, 0]
    );
}

#[test]
fn wrong1() {
    let n = 5;
    let lamps = [[0, 0], [0, 1], [0, 4]];
    let queries = [[0, 0], [0, 1], [0, 2]];
    let lamps: Vec<Vec<i32>> = lamps.into_iter().map(Vec::from).collect();
    let queries: Vec<Vec<i32>> = queries.into_iter().map(Vec::from).collect();
    assert_eq!(
        Solution::grid_illumination(n, lamps, queries),
        vec![1, 1, 1]
    );
}
