use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

type Link = Option<Rc<RefCell<TreeNode>>>;

pub fn lca_deepest_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(x: Link, dep: u32) -> Option<(u32, Link)> {
        // println!("{:#?} : {}", x, dep);
        
        x.map(|t| {
            
            let lc = dfs(t.as_ref().borrow().left.clone(), dep + 1);
            let rc = dfs(t.as_ref().borrow().right.clone(), dep + 1);

            // println!("val = {:#?}", t.as_ref().borrow().val);
            // println!("lc = {:#?}", lc);
            // println!("rc = {:#?}", rc);

            if let Some(l) = lc {
                if let Some(r) = rc {
                    match l.0.cmp(&r.0) {
                        std::cmp::Ordering::Less => Some(r),
                        std::cmp::Ordering::Greater => Some(l),
                        _ => Some((l.0, Some(t))),
                    }    
                } else {
                    Some(l)
                }
            } else if rc.is_none(){
                Some((dep, Some(t)))
            } else {
                rc
            }
        }).flatten()
    }
    if let Some((d, link)) = dfs(root, 0) {
        link
    } else {
        panic!("something wrong!")
    }
}

#[test]
fn example() {
    let n6 = TreeNode::new(6);
    let n7 = TreeNode::new(7);
    let n4 = TreeNode::new(4);
    let n0 = TreeNode::new(0);
    let n8 = TreeNode::new(8);
    let l2 = Some(Rc::new(RefCell::new(TreeNode{
        val: 2,
        left: Some(Rc::new(RefCell::new(n7))),
        right: Some(Rc::new(RefCell::new(n4)))
    })));
    let l1 = Some(Rc::new(RefCell::new(TreeNode{
        val: 1,
        left: Some(Rc::new(RefCell::new(n0))),
        right: Some(Rc::new(RefCell::new(n8)))
    })));
    let l5 = Some(Rc::new(RefCell::new(TreeNode{
        val: 5,
        left: Some(Rc::new(RefCell::new(n6))),
        right: l2,
    })));
    let root = Some(Rc::new(RefCell::new(TreeNode{
        val: 3,
        left: l5,
        right: l1,
    })));

    let ans = lca_deepest_leaves(root);
    assert_eq!(ans.unwrap().borrow().val, 2i32);
}
