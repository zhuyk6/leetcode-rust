use std::collections::HashMap;

#[allow(unused)]
pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    (low_limit..=high_limit).for_each(|mut n| {
        let mut acc = 0;
        while n > 0 {
            acc += n % 10;
            n /= 10;
        }
        map.entry(acc).and_modify(|c| *c += 1).or_insert(1);
    });
    *map.values().max().unwrap()
}
