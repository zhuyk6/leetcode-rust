use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

type Link = Option<Rc<RefCell<TreeNode>>>;

#[allow(unused)]
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_vec(vals: Vec<Option<i32>>) -> Link {
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
            if let Some(Some(v)) = children.get(1) {
                parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(*v))));
                que.push_back(parent.borrow().right.clone());
            }
        }
        root
    }

    pub fn to_list(root: Link) -> Vec<String> {
        let mut que: VecDeque<(Link, usize)> = Default::default();
        let mut list: Vec<String> = vec![];

        que.push_back((root.clone(), 0));
        while let Some((t, dep)) = que.pop_front() {
            if let Some(node) = t {
                list.push(node.as_ref().borrow().val.to_string());
                que.push_back((node.as_ref().borrow().left.clone(), dep + 1));
                que.push_back((node.as_ref().borrow().right.clone(), dep + 1));
            } else {
                list.push("null".to_string());
            }
        }
        while let Some(s) = list.last() {
            if s == "null" {
                list.pop();
            } else {
                break;
            }
        }
        list
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

#[allow(unused_imports)]
pub(crate) use tree;

#[test]
fn test_tree_build() {
    let t = tree![1, 2, null, 3];
    println!("{:#?}", t);
}

#[test]
fn test_tolist() {
    let t = tree![1, 2, null, 3];
    let list = TreeNode::to_list(t).join(",");
    let list = format!("[{}]", list);
    println!("{:#?}", list);
}
