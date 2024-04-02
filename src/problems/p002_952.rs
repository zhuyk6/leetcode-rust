struct Solution;

#[allow(unused)]
impl Solution {
    pub fn minimum_added_coins(mut coins: Vec<i32>, target: i32) -> i32 {
        let mut ans = 0;
        let mut cover = 0;

        coins.sort_unstable();

        for v in coins {
            while v > cover + 1 {
                // add (cover+1)
                ans += 1;
                cover = cover + cover + 1;
            }
            // add v
            cover += v;
            if cover >= target {
                break;
            }
        }

        while cover < target {
            // add (cover+1)
            ans += 1;
            cover = cover + cover + 1;
        }

        ans
    }
}
