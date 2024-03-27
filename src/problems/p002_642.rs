struct Graph {
    dis: Vec<Vec<i32>>,
}

#[allow(unused, clippy::needless_range_loop)]
impl Graph {
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let n = n as usize;
        let mut dis = vec![vec![i32::MAX; n]; n];

        for i in 0..n {
            dis[i][i] = 0;
        }

        for e in edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            let w = e[2];
            dis[x][y] = w;
        }

        for k in 0..n {
            for i in (0..n).filter(|i| *i != k) {
                for j in (0..n).filter(|j| *j != i && *j != k) {
                    dis[i][j] = dis[i][j].min(dis[i][k].saturating_add(dis[k][j]));
                }
            }
        }

        Graph { dis }
    }

    fn add_edge(&mut self, edge: Vec<i32>) {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        let w = edge[2];
        let n = self.dis.len();

        for x in 0..n {
            for y in 0..n {
                self.dis[x][y] = self.dis[x][y]
                    .min(self.dis[x][u].saturating_add(self.dis[v][y].saturating_add(w)));
            }
        }
    }

    fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        let x = node1 as usize;
        let y = node2 as usize;
        match self.dis[x][y] {
            i32::MAX => -1,
            d => d,
        }
    }
}
