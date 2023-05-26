use std::collections::{HashSet, HashMap};


pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
    let mut groups = HashMap::new();

    for v in nums {
        groups.entry(v % k).or_insert(vec![]).push(v);
    }

    fn cal(mut v: Vec<i32>, k: i32) -> i32 {
        v.sort();
        // println!("v: {:?}", v);

        let mut arr: Vec<(i32, u32)> = vec![];
        for x in v {
            if !arr.is_empty() && arr.last().unwrap().0 == x {
                arr.last_mut().unwrap().1 += 1;
            } else {
                arr.push((x, 1));
            }
        }
        println!("arr: {:?}", arr);
        
        let n = arr.len();
        let mut f = vec![0; n+1];
        f[0] = 1;
        f[1] = 2_i32.pow(arr[0].1);
        for i in 2..=n {
            // not select i
            f[i] = f[i-1];
            // select i
            if arr[i-1].0 - arr[i-2].0 == k {
                f[i] += f[i-2] * (2_i32.pow(arr[i-1].1) - 1);
            } else {
                f[i] += f[i-1] * (2_i32.pow(arr[i-1].1) - 1);
            }
        }
        println!("f: {:?}", f);
        f[n]
    }

    groups.into_iter()
        .map(|(_, v)| cal(v, k))
        .product::<i32>() - 1
}

#[test]
fn example() {
    let nums = vec![2, 6, 4];
    let k = 2;
    assert_eq!(beautiful_subsets(nums, k), 4);

    let nums = vec![1];
    let k = 1;
    assert_eq!(beautiful_subsets(nums, k), 1);
}

#[test]
fn wrong() {
    let nums = vec![10,4,5,7,2,1];
    let k = 3;
    assert_eq!(beautiful_subsets(nums, k), 23);
}

#[test]
fn wrong2() {
    let nums = vec![1,2,3,3];
    let k = 1;
    assert_eq!(beautiful_subsets(nums, k), 8);
}

#[test]
fn wrong3() {
    let nums = vec![2,2,4,4,6,6];
    let k = 2;
    assert_eq!(beautiful_subsets(nums, k), 18);
}

#[test]
fn timelimit() {
    let nums = vec![942,231,247,267,741,320,844,276,578,659,96,697,801,892,752,948,176,92,469,595];
    let k = 473;
    assert_eq!(beautiful_subsets(nums, k), 786431);
}
