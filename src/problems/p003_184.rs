struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
        let mut cnt = [0; 24];
        for h in hours {
            let tmp = (h as usize) % 24;
            cnt[tmp] += 1;
        }
        let mut ans = cnt[0] * (cnt[0] - 1) / 2;
        ans += cnt[12] * (cnt[12] - 1) / 2;
        for i in 1..12 {
            let j = 24 - i;
            ans += cnt[i] * cnt[j];
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let hours = vec![12, 12, 30, 24, 24];
        assert_eq!(Solution::count_complete_day_pairs(hours), 2);
    }

    #[test]
    fn sample2() {
        let hours = vec![72, 48, 24, 3];
        assert_eq!(Solution::count_complete_day_pairs(hours), 3);
    }
}
