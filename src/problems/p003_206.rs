pub struct Solution;

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
        let it0 = colors.iter().cycle();
        let mut it1 = it0.clone();
        it1.next();
        let mut it2 = it1.clone();
        it2.next();

        let n = colors.len();
        it0.zip(it1)
            .zip(it2)
            .take(n)
            .filter(|&((&x, &y), &z)| x != y && y != z)
            .count() as i32
    }
}
