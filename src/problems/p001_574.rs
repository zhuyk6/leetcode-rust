pub struct Solution;

impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut ans = n - 1;

        let mut j = n - 1;
        while j > 0 && arr[j - 1] <= arr[j] {
            j -= 1;
        }
        ans = ans.min(j);
        println!("j = {}", j);

        let mut i = 0usize;
        loop {
            while j < n && arr[i] > arr[j] {
                j += 1;
            }
            ans = ans.min(j.saturating_sub(i + 1));
            if i + 1 >= n || arr[i] > arr[i + 1] {
                break;
            }
            i += 1;
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example() {
        let arr: Vec<_> = [1, 2, 3, 10, 4, 2, 3, 5].into_iter().collect();
        assert_eq!(Solution::find_length_of_shortest_subarray(arr), 3);

        let arr: Vec<_> = [5, 4, 3, 2, 1].into_iter().collect();
        assert_eq!(Solution::find_length_of_shortest_subarray(arr), 4);

        let arr = vec![1, 2, 3];
        assert_eq!(Solution::find_length_of_shortest_subarray(arr), 0);
    }

    #[test]
    fn test1() {
        let arr = vec![6, 3, 10, 11, 15, 20, 13, 3, 18, 12];
        assert_eq!(Solution::find_length_of_shortest_subarray(arr), 8);
    }
}
