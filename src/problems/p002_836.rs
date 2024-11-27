pub struct Solution;

#[derive(Debug, Clone)]
enum Tag {
    Untag,
    Circle {
        idx: usize,
        belong: usize,
    },
    Tree {
        #[allow(unused)]
        fa: usize,
        dep: usize,
    },
}

fn dfs_label(
    vis_id: u32,
    x: usize,
    next: &[usize],
    vis: &mut [u32],
    tags: &mut [Tag],
    circles: &mut Vec<Vec<usize>>,
) {
    vis[x] = vis_id;
    let y = next[x];
    if vis[y] == u32::MAX {
        // never visit
        dfs_label(vis_id, y, next, vis, tags, circles);
        if let Tag::Untag = &tags[x] {
            match &tags[y] {
                Tag::Untag => panic!("This should not happen!"),
                Tag::Circle { .. } => {
                    tags[x] = Tag::Tree { fa: y, dep: 1 };
                }
                Tag::Tree { dep, .. } => {
                    tags[x] = Tag::Tree {
                        fa: y,
                        dep: *dep + 1,
                    }
                }
            }
        }
    } else if vis[y] != vis_id {
        // visit before
        match &tags[y] {
            Tag::Untag => panic!("This should not happen!"),
            Tag::Circle { .. } => {
                tags[x] = Tag::Tree { fa: y, dep: 1 };
            }
            Tag::Tree { dep, .. } => {
                tags[x] = Tag::Tree {
                    fa: y,
                    dep: *dep + 1,
                }
            }
        }
    } else {
        // Find a circle
        let belong = circles.len();
        let mut idx = 0;

        let mut circ = vec![y];
        let mut cur = y;
        tags[cur] = Tag::Circle { idx, belong };

        while cur != x {
            cur = next[cur];
            circ.push(cur);
            idx += 1;
            tags[cur] = Tag::Circle { idx, belong };
        }
        circles.push(circ);
    }
}

const H: usize = 17;

struct Tree {
    fa: Vec<[usize; H]>,
    prefix_sum: Vec<usize>,
}

impl Tree {
    fn dfs(
        sons: &[Vec<usize>],
        x: usize,
        father: usize,
        mut acc: usize,
        fa: &mut [[usize; H]],
        prefix_sum: &mut [usize],
    ) {
        acc += x;
        prefix_sum[x] = acc;

        fa[x][0] = father;
        for h in 1..H {
            let y = fa[x][h - 1];
            if y != usize::MAX {
                fa[x][h] = fa[y][h - 1];
            } else {
                break;
            }
        }
        for &y in &sons[x] {
            Tree::dfs(sons, y, x, acc, fa, prefix_sum);
        }
    }

    fn get_fa(&self, mut x: usize, mut k: usize) -> usize {
        let mut h = H - 1;
        while k > 0 {
            if k >= (1 << h) {
                if self.fa[x][h] != usize::MAX {
                    x = self.fa[x][h];
                    k -= 1 << h;
                } else {
                    return usize::MAX;
                }
            } else {
                h -= 1;
            }
        }
        x
    }
}

struct Circle {
    size: usize,
    #[allow(unused)]
    arr: Vec<usize>,
    prefix_sum: Vec<usize>,
}

impl Circle {
    fn with_arr(arr: Vec<usize>) -> Self {
        let size = arr.len();
        let mut prefix_sum = vec![0; size];
        prefix_sum[0] = arr[0];
        for i in 1..size {
            prefix_sum[i] = prefix_sum[i - 1] + arr[i];
        }
        Self {
            size,
            arr,
            prefix_sum,
        }
    }

    fn query(&self, x: usize, k: usize) -> usize {
        // eprintln!("query circle {x} with {k}");

        let n = self.size;
        let (p, q) = (k / n, k % n);
        let mut ans = p * self.prefix_sum[n - 1];

        if x + q < n {
            ans +=
                self.prefix_sum[x + q] - x.checked_sub(1).map(|i| self.prefix_sum[i]).unwrap_or(0);
        } else {
            ans += self.prefix_sum[n - 1] - self.prefix_sum[x - 1] + self.prefix_sum[(x + q) % n];
        }

        ans
    }
}

fn query_answer(
    x: usize,
    tree: &Tree,
    circles: &[Circle],
    next: &[usize],
    tags: &[Tag],
    k: usize,
) -> usize {
    // eprintln!("Query start: {x}");

    match &tags[x] {
        Tag::Untag => panic!("Impossible!"),
        Tag::Circle { idx, belong } => circles[*belong].query(*idx, k),
        Tag::Tree { fa: _, dep } => {
            if *dep > k {
                // only in the tree
                let y = tree.get_fa(x, k);
                match tree.get_fa(y, 1) {
                    usize::MAX => tree.prefix_sum[x],
                    z => tree.prefix_sum[x] - tree.prefix_sum[z],
                }
            } else {
                // tree and circle
                let mut ans = tree.prefix_sum[x];
                let k = k - *dep;
                let root = tree.get_fa(x, *dep - 1);

                // eprintln!("tree = {ans}");

                if let Tag::Circle { idx, belong } = &tags[next[root]] {
                    let tmp = circles[*belong].query(*idx, k);
                    // eprintln!("circle = {tmp}");
                    ans += tmp;
                } else {
                    panic!("impossible!");
                }

                ans
            }
        }
    }
}

impl Solution {
    pub fn get_max_function_value(receiver: Vec<i32>, k: i64) -> i64 {
        let n = receiver.len();
        let next: Vec<usize> = receiver.into_iter().map(|v| v as usize).collect();
        let mut circles = vec![];
        let mut tags = vec![Tag::Untag; n];
        {
            let mut vis = vec![u32::MAX; n];
            let mut vis_id = 0;
            for x in 0..n {
                if vis[x] == u32::MAX {
                    dfs_label(vis_id, x, &next, &mut vis, &mut tags, &mut circles);
                    vis_id += 1;
                }
            }

            // dbg!(&circles);
            // dbg!(&tags);
        }

        // build circles
        let circles: Vec<Circle> = circles.into_iter().map(Circle::with_arr).collect();

        // build the tree
        let tree = {
            let mut sons = vec![vec![]; n];
            let mut fa = vec![[usize::MAX; H]; n];
            let mut prefix_sum = vec![0; n];

            #[allow(clippy::needless_range_loop)]
            for i in 0..n {
                let j = next[i];
                if let Tag::Tree { .. } = &tags[j] {
                    sons[j].push(i);
                    fa[i][0] = j;
                }
            }

            for i in 0..n {
                if let Tag::Tree { .. } = &tags[i] {
                    if fa[i][0] == usize::MAX {
                        Tree::dfs(&sons, i, usize::MAX, 0, &mut fa, &mut prefix_sum);
                    }
                }
            }

            Tree { fa, prefix_sum }
        };

        (0..n)
            .map(|x| query_answer(x, &tree, &circles, &next, &tags, k as usize))
            .max()
            .unwrap() as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let receiver = vec![2, 0, 1];
        let k = 4;
        assert_eq!(Solution::get_max_function_value(receiver, k), 6);
    }

    #[test]
    fn sample2() {
        let receiver = vec![1, 1, 1, 2, 3];
        let k = 3;
        assert_eq!(Solution::get_max_function_value(receiver, k), 10);
    }

    #[test]
    fn sample3() {
        let receiver = vec![1, 2, 3, 4, 5, 3, 4, 2];
        let k = 10;
        assert_eq!(Solution::get_max_function_value(receiver, k), 46);
    }
}
