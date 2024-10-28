pub struct Solution;

impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort_unstable();
        assert!(*people.last().unwrap() <= limit);

        let mut ans = 0;
        let mut j = people.len() - 1;
        let mut i = 0;
        while i < j {
            if people[i] + people[j] > limit {
                ans += 1;
                j -= 1;
            } else {
                ans += 1;
                i += 1;
                j -= 1;
            }
        }
        if i == j {
            ans += 1;
        }
        ans
    }
}
