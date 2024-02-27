const MAX_ITER: usize = 30;

struct TreeAncestor {
    fa: Vec<[Option<usize>; MAX_ITER]>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TreeAncestor {
    #[allow(unused)]
    fn new(n: i32, parent: Vec<i32>) -> Self {
        let n = n as usize;
        let mut fa = vec![[None; MAX_ITER]; n];
        let mut son = vec![vec![]; n];

        for (i, p) in parent.into_iter().enumerate() {
            if let Ok(p) = usize::try_from(p) {
                son[p].push(i);
            }
        }

        fn dfs(x: usize, father: Option<usize>, son: &Vec<Vec<usize>>, fa: &mut Vec<[Option<usize>; MAX_ITER]>) {
            fa[x][0] = father;
            for i in 1..MAX_ITER {
                fa[x][i] = fa[x][i-1].and_then(|p| fa[p][i-1]);
                if fa[x][i].is_none() {
                    break;
                }
            }
            for &y in &son[x] {
                dfs(y, Some(x), son, fa);
            }
        }

        dfs(0, None, &son, &mut fa);

        TreeAncestor { fa }
    }
    
    #[allow(unused)]
    fn get_kth_ancestor(&self, node: i32, k: i32) -> i32 {
        let mut node = Some(node as usize);
        for i in (0..MAX_ITER).rev() {
            if (k >> i) & 1 > 0 {
                node = node.and_then(|x| self.fa[x][i]);
            }
        }
        node.map(|x| x as i32).unwrap_or(-1)
    }
}

#[test]
fn example() {
    // [[7,[-1,0,0,1,1,2,2]],[3,1],[5,2],[6,3]]
    let t = TreeAncestor::new(7, vec![-1, 0, 0, 1, 1, 2, 2]);

    assert_eq!(t.get_kth_ancestor(3, 1), 1);
    assert_eq!(t.get_kth_ancestor(5, 2), 0);
    assert_eq!(t.get_kth_ancestor(6, 3), -1);
}
