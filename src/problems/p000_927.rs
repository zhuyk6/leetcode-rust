pub struct Solution;

fn calc(arr: &[i32]) -> Option<(usize, usize)> {
    let n = arr.len();

    // find all 1s in arr
    let ones_pos: Vec<usize> = arr
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| if x == 1 { Some(i) } else { None })
        .collect();

    let ones_count = ones_pos.len();
    if ones_count == 0 {
        return Some((0, 2));
    } else if ones_count % 3 != 0 {
        return None;
    }

    // each part must have `num` 1s
    let num = ones_count / 3;
    assert!(num > 0);

    // split the array into 3 parts
    // 1st part: [..=i]
    // 2nd part: [i+1..=j]
    // 3rd part: [j+1..]

    // i in (gap1_left..gap1_right)
    let gap1_right = ones_pos[num];
    let gap1_left = ones_pos[num - 1];

    // j in (gap2_left..gap2_right)
    let gap2_right = ones_pos[ones_count - num];
    let gap2_left = ones_pos[ones_count - num - 1];

    // ending zeros in the 3rd part
    let ending_zeros = n - ones_pos.last().unwrap() - 1;

    if gap1_right - gap1_left - 1 < ending_zeros || gap2_right - gap2_left - 1 < ending_zeros {
        return None;
    }
    let i = gap1_left + ending_zeros;
    let j = gap2_left + ending_zeros;

    fn check(arr1: &[i32], arr2: &[i32]) -> bool {
        let mut i = 0;
        let mut j = 0;
        while arr1[i] == 0 && i < arr1.len() {
            i += 1;
        }
        while arr2[j] == 0 && j < arr2.len() {
            j += 1;
        }

        while i < arr1.len() && j < arr2.len() {
            if arr1[i] != arr2[j] {
                return false;
            }
            i += 1;
            j += 1;
        }

        i == arr1.len() && j == arr2.len()
    }

    if check(&arr[..=i], &arr[gap1_right..=j]) && check(&arr[..=i], &arr[gap2_right..]) {
        Some((i, j + 1))
    } else {
        None
    }
}

impl Solution {
    pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
        if let Some((i, j)) = calc(&arr) {
            vec![i as i32, j as i32]
        } else {
            vec![-1, -1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let arr = vec![1, 0, 1, 0, 1];
        assert_eq!(Solution::three_equal_parts(arr), vec![0, 3]);
    }

    #[test]
    fn sample2() {
        let arr = vec![1, 1, 0, 1, 1];
        assert_eq!(Solution::three_equal_parts(arr), vec![-1, -1]);
    }

    #[test]
    fn sample3() {
        let arr = vec![1, 1, 0, 0, 1];
        assert_eq!(Solution::three_equal_parts(arr), vec![0, 2]);
    }

    #[test]
    fn issue1() {
        let arr = vec![0, 1, 0, 1, 1];
        assert_eq!(Solution::three_equal_parts(arr), vec![1, 4]);
    }
}
