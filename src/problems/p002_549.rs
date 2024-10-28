pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn distinct_integers(n: i32) -> i32 {
        let mut set = HashSet::from([n]);

        loop {
            let mut set2 = set.clone();
            for &x in set.iter() {
                for y in 1..=x {
                    if x % y == 1 {
                        set2.insert(y);
                    }
                }
            }
            if set2.difference(&set).count() == 0 {
                break;
            } else {
                set = set2;
            }
        }

        set.len() as i32
    }
}

#[test]
fn test1() {
    let n = 5;
    assert_eq!(Solution::distinct_integers(n), 4);
}

#[test]
fn test2() {
    let n = 3;
    assert_eq!(Solution::distinct_integers(n), 2);
}

#[test]
fn test3() {
    let n = 10;
    assert_eq!(Solution::distinct_integers(n), 9);
}
