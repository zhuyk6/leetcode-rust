pub struct Solution;

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let n = customers.len();
        let m = minutes as usize;

        let mut prefix_sum1 = vec![0; n];
        prefix_sum1[0] = customers[0];
        for i in 1..n {
            prefix_sum1[i] = prefix_sum1[i - 1] + customers[i];
        }

        let mut prefix_sum2 = vec![0; n];
        prefix_sum2[0] = if grumpy[0] == 0 { customers[0] } else { 0 };
        for i in 1..n {
            prefix_sum2[i] = prefix_sum2[i - 1] + if grumpy[i] == 0 { customers[i] } else { 0 };
        }

        let mut ans = prefix_sum1[m - 1] + prefix_sum2[n - 1] - prefix_sum2[m - 1];

        for i in 1..n {
            let c1 = prefix_sum1[(i + m - 1).min(n - 1)] - prefix_sum1[i - 1];
            let c2 = prefix_sum2[i - 1] + prefix_sum2[n - 1] - prefix_sum2[(i + m - 1).min(n - 1)];
            ans = ans.max(c1 + c2);
        }

        ans
    }
}

#[test]
fn tes1() {
    let customers = vec![1, 0, 1, 2, 1, 1, 7, 5];
    let grumpy = vec![0, 1, 0, 1, 0, 1, 0, 1];
    let minutes = 3;
    assert_eq!(Solution::max_satisfied(customers, grumpy, minutes), 16);
}

#[test]
fn tes2() {
    let customers = vec![1];
    let grumpy = vec![0];
    let minutes = 1;
    assert_eq!(Solution::max_satisfied(customers, grumpy, minutes), 1);
}
