struct Solution;

fn dfs(arr: &mut [i32], tmp: &mut [i32], vis: &mut [bool], ans: &mut Vec<i32>) {
    let n = arr.len();
    if n == 1 {
        return;
    }

    // initialize vis
    for b in &mut *vis {
        *b = false;
    }

    let d = arr[1] - arr[0];
    let (arr_s, arr_t) = tmp.split_at_mut(n / 2);

    let mut i = 0;
    let mut j = 0;
    let mut num_s = 0;
    let mut num_t = 0;
    while i < n {
        if !vis[i] {
            vis[i] = true;
            arr_s[num_s] = arr[i];
            num_s += 1;

            j = j.max(i + 1);
            while j < n && arr[j] < arr[i] + d {
                j += 1;
            }
            if j < n && arr[j] == arr[i] + d {
                vis[j] = true;
                arr_t[num_t] = arr[j];
                num_t += 1;
                j += 1;
            } else {
                panic!("important!");
            }
        }
        i += 1;
    }

    // copy arr from arr_s and arr_t
    arr[..num_s].copy_from_slice(arr_s);
    arr[num_s..].copy_from_slice(arr_t);

    // if s countains 0, then d in the answer, otherwise -d in the answer.
    if arr_s.iter().filter(|x| **x == 0).count() > 0 {
        ans.push(d);
        dfs(
            &mut arr[..num_s],
            &mut tmp[..(n / 2)],
            &mut vis[..(n / 2)],
            ans,
        );
    } else {
        ans.push(-d);
        dfs(
            &mut arr[num_s..],
            &mut tmp[..(n / 2)],
            &mut vis[..(n / 2)],
            ans,
        );
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn recover_array(_: i32, sums: Vec<i32>) -> Vec<i32> {
        let mut arr = sums;
        arr.sort();
        let n = arr.len();
        let mut tmp = vec![0; n];
        let mut vis = vec![false; n];
        let mut ans = Vec::new();
        dfs(&mut arr[..], &mut tmp[..], &mut vis[..], &mut ans);
        ans
    }
}

#[test]
fn test1() {
    let n = 3;
    let sums = vec![-3, -2, -1, 0, 0, 1, 2, 3];
    let mut ans = Solution::recover_array(n, sums);
    ans.sort();
    assert_eq!(ans, vec![-3, 1, 2]);
}
#[test]
fn test2() {
    let n = 2;
    let sums = vec![0, 0, 0, 0];
    let mut ans = Solution::recover_array(n, sums);
    ans.sort();
    assert_eq!(ans, vec![0, 0]);
}
#[test]
fn test3() {
    let n = 4;
    let sums = vec![0, 0, 5, 5, 4, -1, 4, 9, 9, -1, 4, 3, 4, 8, 3, 8];
    let mut ans = Solution::recover_array(n, sums);
    ans.sort();
    assert_eq!(ans, vec![-1, 0, 4, 5]);
}
