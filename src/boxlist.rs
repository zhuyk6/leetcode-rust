#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[macro_export]
macro_rules! list {
    [] => {
        None
    };
    [$val:expr_2021 $(, $rest:expr_2021)*] => {
        {
            let mut head = Some(Box::new(ListNode::new($val)));
            head.as_mut().map(|node| node.next = list!($($rest),*));
            head
        }
    };
}

#[allow(unused_imports)]
pub(crate) use list;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(list![], Option::<Box<ListNode>>::None);

        let list = list!(1, 2, 3);
        println!("{list:?}");
    }
}
