use std::rc::Rc;

enum Node {
    Leaf(LeafNode),
    Inter(InterNode),
}

use Node::*;

struct LeafNode {
    _idx: usize,
    val: i32,
}

struct InterNode {
    left: usize,
    right: usize,
    lc: Rc<Node>,
    rc: Rc<Node>,
}

fn build(l: usize, r: usize) -> Rc<Node> {
    if l == r {
        Rc::new(Leaf(LeafNode { _idx: l, val: 0 }))
    } else {
        let mid = (l + r) >> 1;
        let lc = build(l, mid);
        let rc = build(mid + 1, r);
        Rc::new(Inter(InterNode {
            left: l,
            right: r,
            lc,
            rc,
        }))
    }
}

fn update(x: &Rc<Node>, idx: usize, val: i32) -> Rc<Node> {
    match x.as_ref() {
        Leaf(_) => Rc::new(Leaf(LeafNode { _idx: idx, val })),
        Inter(inter) => {
            let mid = (inter.left + inter.right) >> 1;
            if idx <= mid {
                Rc::new(Inter(InterNode {
                    left: inter.left,
                    right: inter.right,
                    lc: update(&inter.lc, idx, val),
                    rc: inter.rc.clone(),
                }))
            } else {
                Rc::new(Inter(InterNode {
                    left: inter.left,
                    right: inter.right,
                    lc: inter.lc.clone(),
                    rc: update(&inter.rc, idx, val),
                }))
            }
        }
    }
}

fn query(x: &Rc<Node>, idx: usize) -> i32 {
    match x.as_ref() {
        Leaf(leaf) => leaf.val,
        Inter(inter) => {
            let mid = (inter.left + inter.right) >> 1;
            if idx <= mid {
                query(&inter.lc, idx)
            } else {
                query(&inter.rc, idx)
            }
        }
    }
}

struct SnapshotArray {
    snaps: Vec<Rc<Node>>,
}

#[allow(dead_code)]
impl SnapshotArray {
    fn new(length: i32) -> Self {
        let mut v = vec![];
        let root = build(0, length as usize);
        v.push(root);
        SnapshotArray { snaps: v }
    }

    fn set(&mut self, index: i32, val: i32) {
        let root = self.snaps.last().unwrap();
        let root = update(root, index as usize, val);
        *self.snaps.last_mut().unwrap() = root;
    }

    fn snap(&mut self) -> i32 {
        let ans = self.snaps.len() - 1;

        let root = self.snaps.last_mut().unwrap().clone();
        self.snaps.push(root);

        ans as i32
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let root = self.snaps.get(snap_id as usize).unwrap();
        query(root, index as usize)
    }
}
