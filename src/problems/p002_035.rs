struct Solution;

struct Dfs {
    n: usize,
    nums: Vec<i32>,
    sum: i32,
    ans: i32,
}

#[allow(dead_code)]
impl Dfs {
    fn new(nums: Vec<i32>) -> Self {
        let sum: i32 = nums.iter().sum();
        let n = nums.len();
        Dfs {
            n,
            nums,
            sum,
            ans: i32::MAX,
        }
    }

    fn dfs(&mut self, x: usize, left: usize, acc: i32) {
        if left == 0 {
            self.ans = self.ans.min((self.sum - 2 * acc).abs());
        } else if x < self.n && self.n - x >= left {
            self.dfs(x + 1, left - 1, acc + self.nums[x]);
            self.dfs(x + 1, left, acc);
        }
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len() / 2;
        let mut map = vec![vec![]; 1 << n];

        for mask in 0..(1u32 << n) {
            let cnt = mask.count_ones() as usize;
            let mut acc = 0;
            for (i, &v) in nums[..n].iter().enumerate() {
                if mask & (1 << i) > 0 {
                    acc += v;
                } else {
                    acc -= v;
                }
            }
            map[cnt].push(acc);
        }

        for v in &mut map {
            v.sort_unstable();
        }

        let mut ans = i32::MAX;
        for mask in 0..(1u32 << n) {
            let cnt = mask.count_ones();
            let mut acc = 0;
            for (i, &v) in nums[n..].iter().enumerate() {
                if mask & (1 << i) > 0 {
                    acc += v;
                } else {
                    acc -= v;
                }
            }
            let v = &map[n - cnt as usize];
            let p = v.partition_point(|&x| x + acc < 0);
            if p > 0 {
                ans = ans.min((v[p - 1] + acc).abs());
            }
            if p < v.len() {
                ans = ans.min((v[p] + acc).abs());
            }
        }

        ans
    }
}

#[test]
fn test1() {
    let nums = vec![3, 9, 7, 3];
    assert_eq!(Solution::minimum_difference(nums), 2);
}

#[test]
fn test2() {
    let nums = vec![-36, 36];
    assert_eq!(Solution::minimum_difference(nums), 72);
}

#[test]
fn test3() {
    let nums = vec![2, -1, 0, 4, -2, -9];
    assert_eq!(Solution::minimum_difference(nums), 0);
}

#[test]
fn time_limited() {
    let nums = vec![
        7772197, 4460211, -7641449, -8856364, 546755, -3673029, 527497, -9392076, 3130315,
        -5309187, -4781283, 5919119, 3093450, 1132720, 6380128, -3954678, -1651499, -7944388,
        -3056827, 1610628, 7711173, 6595873, 302974, 7656726, -2572679, 0, 2121026, -5743797,
        -8897395, -9699694,
    ];
    assert_eq!(Solution::minimum_difference(nums), 1);
}
