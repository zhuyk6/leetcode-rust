pub struct Solution;

impl Solution {
    pub fn max_building(n: i32, mut restrictions: Vec<Vec<i32>>) -> i32 {
        restrictions.sort_unstable_by_key(|w| w[0]);

        let mut stack = Vec::new();
        stack.push((1, 0));

        for w in restrictions {
            let p = w[0];
            let v = w[1];
            if v >= stack.last().unwrap().1 + (p - stack.last().unwrap().0) {
                continue;
            }
            while v <= stack.last().unwrap().1 - (p - stack.last().unwrap().0) {
                stack.pop();
            }
            stack.push((p, v));
        }

        // println!("{:#?}", stack);

        let mut ans = 0;
        let mut last = stack[0];
        for &(p, v) in &stack[1..] {
            let y = ((p - last.0) + last.1 + v) / 2;
            ans = ans.max(y);
            last = (p, v);
        }
        ans = ans.max((n - last.0) + last.1);

        ans
    }
}

#[test]
fn test1() {
    let n = 5;
    let restrictions = [[2, 1], [4, 1]];
    let restrictions: Vec<Vec<i32>> = restrictions.into_iter().map(Vec::from).collect();
    assert_eq!(Solution::max_building(n, restrictions), 2);
}

#[test]
fn test2() {
    let n = 6;
    let restrictions: [[i32; 2]; 0] = [];
    let restrictions: Vec<Vec<i32>> = restrictions.into_iter().map(Vec::from).collect();
    assert_eq!(Solution::max_building(n, restrictions), 5);
}

#[test]
fn test3() {
    let n = 10;
    let restrictions = [[5, 3], [2, 5], [7, 4], [10, 3]];
    let restrictions: Vec<Vec<i32>> = restrictions.into_iter().map(Vec::from).collect();
    assert_eq!(Solution::max_building(n, restrictions), 5);
}

#[test]
fn wrong1() {
    let n = 100;
    let restrictions = [
        [68, 29],
        [89, 27],
        [66, 26],
        [34, 9],
        [53, 23],
        [93, 24],
        [70, 12],
        [25, 24],
        [5, 4],
        [94, 41],
        [51, 42],
        [6, 39],
        [55, 21],
        [69, 9],
        [39, 50],
        [99, 42],
        [77, 24],
        [81, 46],
        [90, 43],
        [27, 14],
        [31, 5],
        [67, 37],
        [82, 10],
        [26, 47],
        [84, 34],
        [37, 30],
        [83, 39],
        [21, 39],
        [49, 39],
        [13, 48],
        [12, 34],
        [57, 0],
        [7, 43],
        [17, 6],
        [23, 0],
        [86, 30],
        [47, 30],
        [61, 19],
        [30, 49],
        [95, 42],
        [3, 31],
        [33, 36],
        [11, 45],
        [75, 39],
        [85, 46],
        [29, 33],
        [2, 9],
        [22, 17],
        [65, 42],
        [96, 0],
        [35, 46],
        [88, 47],
        [74, 0],
        [73, 47],
        [41, 45],
        [15, 21],
        [97, 0],
        [64, 0],
        [40, 21],
        [76, 2],
        [54, 3],
        [24, 33],
        [45, 24],
        [16, 23],
        [91, 14],
        [43, 35],
        [79, 6],
        [46, 30],
        [71, 3],
        [9, 39],
        [50, 21],
        [48, 45],
        [63, 42],
        [58, 3],
        [10, 26],
        [4, 6],
        [52, 19],
        [32, 39],
        [87, 50],
        [8, 48],
        [19, 25],
        [92, 1],
        [28, 21],
        [59, 31],
        [72, 24],
        [78, 9],
        [100, 8],
        [60, 20],
        [42, 16],
        [38, 8],
        [62, 31],
        [36, 22],
        [44, 27],
        [14, 45],
        [18, 3],
        [98, 0],
        [20, 1],
        [56, 24],
        [80, 3],
    ];
    let restrictions: Vec<Vec<i32>> = restrictions.into_iter().map(Vec::from).collect();
    assert_eq!(Solution::max_building(n, restrictions), 13);
}
