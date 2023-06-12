use std::fs::remove_dir;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[allow(unused)]
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from(v: &[i32]) -> Option<Box<ListNode>> {
        match v.len() {
            0 => None,
            _ => Some(Box::new(ListNode { val: v[0], next: ListNode::from(&v[1..]) }))
        }
    }
}

struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn remove_zero_sum_sublists(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut arr = vec![0];
        while let Some(node) = head {
            arr.push(node.val);
            head = node.next;
        }

        use std::collections::HashMap;

        let mut map: HashMap<i32, usize> = HashMap::new();
        let mut acc = 0;
        let mut sum = vec![];
        for (i, &v) in arr.iter().enumerate() {
            acc += v;
            map.insert(acc, i);
            sum.push(acc);
        }

        println!("arr: {:?}", arr);
        println!("sum: {:?}", sum);
        println!("map: {:?}", map);

        let n = arr.len();
           
        fn dfs(arr: &Vec<i32>, sum: &Vec<i32>, n: usize, cur: usize, map: HashMap<i32, usize>) -> Option<Box<ListNode>> {
            if cur >= n {
                return None;
            }
            let pos = map[&sum[cur]];
            // println!("cur: {}, pos: {}", cur, pos);
            let next = dfs(arr, sum, n, pos+1, map);
            Some(Box::new(ListNode { val: arr[cur], next }))
        }

        dfs(&arr, &sum, n, 0, map).unwrap().next
    }
}

fn show(mut head: Option<Box<ListNode>>) {
    print!("[");
    while let Some(node) = head {
        print!("{}, ", node.val);
        head = node.next;
    }
    println!("]");
}

#[test]
fn example() {
    let head = ListNode::from(&[3,2,1,-1,-2, -3]);
    let head = Solution::remove_zero_sum_sublists(head);
    show(head);
}
