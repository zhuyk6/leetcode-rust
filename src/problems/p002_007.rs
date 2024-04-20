struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_original_array(mut changed: Vec<i32>) -> Vec<i32> {
        changed.sort_unstable();

        let n = changed.len();
        let mut used = vec![false; n];
        let mut ans = vec![];

        let mut j = 1;
        for (i, &v) in changed.iter().enumerate() {
            if used[i] {
                continue;
            }
            println!("i = {i}, v = {v}");
            j = j.max(i + 1);
            while j < n && changed[j] < v * 2 {
                j += 1;
            }
            println!("j = {j}");
            if j < n && changed[j] == v * 2 {
                ans.push(v);
                used[j] = true;
                j += 1;
            } else {
                ans.clear();
                break;
            }
        }

        ans
    }
}

#[test]
fn test1() {
    let changed = vec![1, 3, 4, 2, 6, 8];
    assert_eq!(Solution::find_original_array(changed), vec![1, 3, 4]);
}

#[test]
fn test2() {
    let changed = vec![6, 3, 0, 1];
    assert_eq!(Solution::find_original_array(changed), vec![]);
}

#[test]
fn test3() {
    let changed = vec![1];
    assert_eq!(Solution::find_original_array(changed), vec![]);
}

#[test]
fn test4() {
    let changed = vec![2, 1, 2, 4, 2, 4];
    assert_eq!(Solution::find_original_array(changed), vec![1, 2, 2]);
}
