pub struct Solution;

const P: i64 = 1_000_000_007;

#[inline]
fn pow(mut x: i64, n: i64) -> i64 {
    let mut acc = 1;
    let mut b = 1;
    while b <= n {
        if n & b > 0 {
            acc = acc * x % P;
        }
        b <<= 1;
        x = x * x % P;
    }
    acc
}

impl Solution {
    pub fn count_ways(mut ranges: Vec<Vec<i32>>) -> i32 {
        ranges.sort_unstable_by(|w1, w2| match w1[0].cmp(&w2[0]) {
            std::cmp::Ordering::Equal => w1[1].cmp(&w2[1]),
            t => t,
        });

        println!("ranges: {ranges:?}");

        let mut nums = 0;
        let mut group_right = i32::MIN;

        for w in ranges {
            if w[0] > group_right {
                nums += 1;
                group_right = w[1];
            } else {
                group_right = group_right.max(w[1]);
            }
        }

        pow(2, nums) as i32
    }
}

#[test]
fn test2() {
    let ranges = [[1, 3], [10, 20], [2, 5], [4, 8]];
    let ranges: Vec<Vec<i32>> = ranges.into_iter().map(Vec::from).collect();
    assert_eq!(Solution::count_ways(ranges), 4);
}

#[test]
fn test3() {
    let ranges = [
        [34, 56],
        [28, 29],
        [12, 16],
        [11, 48],
        [28, 54],
        [22, 55],
        [28, 41],
        [41, 44],
    ];
    let ranges: Vec<Vec<i32>> = ranges.into_iter().map(Vec::from).collect();
    assert_eq!(Solution::count_ways(ranges), 2);
}
