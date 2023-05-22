use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

type Link = Option<Rc<RefCell<TreeNode>>>;

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    fn from_vec(vals: Vec<Option<i32>>) -> Link {
        use std::collections::VecDeque;

        let mut que: VecDeque<Link> = VecDeque::new();
        let root = Some(Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap()))));
        que.push_back(root.clone());

        for children in vals[1..].chunks(2) {
            let parent = que.pop_front().unwrap().unwrap();
            
            if let Some(v) = children[0] {
                parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(v))));
                que.push_back(parent.borrow().left.clone());
            }
            if let Some(v) = children.get(1) {
                if let Some(v) = v {
                    parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(*v))));
                    que.push_back(parent.borrow().right.clone());
                }
            }
        }
        root
    }
}

#[macro_export]
macro_rules! tree {
    () => {
        None
    };
    ($($e:expr),*) => {
        {
            let vec = vec![$(stringify!($e)), *];
            println!("{:?}", vec);
            let vec = vec.into_iter().map(|v| v.parse::<i32>().ok()).collect::<Vec<_>>();
            TreeNode::from_vec(vec)
        }
    };
    // ($($e:expr,)*) => {(tree![$($e),*])};
}

#[test]
fn test_tree() {
    let t = tree![1,2,null,3];
    println!("{:#?}", t);
}
