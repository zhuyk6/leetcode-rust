pub struct Solution;

impl Solution {
    pub fn max_path_length(coordinates: Vec<Vec<i32>>, k: i32) -> i32 {
        fn max_increase_path_length(mut arr: Vec<Vec<i32>>) -> i32 {
            arr.sort_unstable_by(|p, q| match p[0].cmp(&q[0]) {
                std::cmp::Ordering::Equal => p[1].cmp(&q[1]),
                ord => ord,
            });

            // dbg!(&arr);

            let groups: Vec<Vec<i32>> = {
                let mut groups = vec![];
                let mut g = vec![];
                let mut cur = i32::MIN;
                for p in arr.into_iter() {
                    if p[0] > cur {
                        if !g.is_empty() {
                            groups.push(g);
                        }
                        cur = p[0];
                        g = vec![];
                    }
                    g.push(p[1]);
                }
                if !g.is_empty() {
                    groups.push(g);
                }
                groups
            };

            // dbg!(&groups);
            if groups.is_empty() {
                return 0;
            }

            let mut ans = 0;
            let mut f: Vec<i32> = vec![i32::MAX; groups.len() + 1];
            f[0] = i32::MIN;
            for g in groups {
                let mut updates = Vec::with_capacity(g.len());
                for v in g {
                    let pos = f.partition_point(|&u| u < v);
                    updates.push((pos, v));
                    ans = ans.max(pos);
                }
                for (pos, v) in updates.into_iter() {
                    f[pos] = f[pos].min(v);
                }
            }

            ans as i32
        }

        let p = coordinates[k as usize].clone();
        let (arr1, arr2) =
            coordinates
                .into_iter()
                .fold((vec![], vec![]), |(mut arr1, mut arr2), q| {
                    match (q[0].cmp(&p[0]), q[1].cmp(&p[1])) {
                        (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => arr1.push(q),
                        (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => arr2.push(q),
                        _ => {}
                    }
                    (arr1, arr2)
                });

        max_increase_path_length(arr1) + max_increase_path_length(arr2) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let coordinates = nested_vec![[3, 1], [2, 2], [4, 1], [0, 0], [5, 3]];
        let k = 1;
        assert_eq!(Solution::max_path_length(coordinates, k), 3)
    }

    #[test]
    fn sample2() {
        let coordinates = nested_vec![[2, 1], [7, 0], [5, 6]];
        let k = 2;
        assert_eq!(Solution::max_path_length(coordinates, k), 2)
    }

    #[test]
    fn sample3() {
        let coordinates = nested_vec![
            [0, 0],
            [0, 1],
            [0, 2],
            [1, 2],
            [1, 3],
            [1, 4],
            [2, 3],
            [2, 5]
        ];
        let k = 0;
        assert_eq!(Solution::max_path_length(coordinates, k), 3)
    }

    #[test]
    fn issue() {
        let coordinates = nested_vec![[5, 4], [7, 8], [0, 4]];
        let k = 1;
        assert_eq!(Solution::max_path_length(coordinates, k), 2)
    }
}
