use std::collections::HashMap;

struct Solution;

#[allow(unused)]
impl Solution {
    pub fn minimum_hamming_distance(
        source: Vec<i32>,
        target: Vec<i32>,
        allowed_swaps: Vec<Vec<i32>>,
    ) -> i32 {
        fn getfa(fa: &mut Vec<usize>, x: usize) -> usize {
            if x != fa[x] {
                fa[x] = getfa(fa, fa[x]);
            }
            fa[x]
        }
        let n = source.len();
        let mut fa: Vec<usize> = (0..n).collect();

        for pair in allowed_swaps.into_iter() {
            let x = getfa(&mut fa, pair[0] as usize);
            let y = getfa(&mut fa, pair[1] as usize);
            fa[x] = y;
        }
        let mut groups = HashMap::new();
        for (i, v) in source.into_iter().enumerate() {
            let gid = getfa(&mut fa, i);
            groups
                .entry(gid)
                .or_insert(HashMap::new())
                .entry(v)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        let mut ans = 0;
        for (i, v) in target.into_iter().enumerate() {
            let gid = getfa(&mut fa, i);
            if let Some(map) = groups.get_mut(&gid) {
                println!("map = {:#?}", map);
                if let Some(cnt) = map.get_mut(&v) {
                    if *cnt > 0 {
                        *cnt -= 1;
                    } else {
                        ans += 1;
                    }
                } else {
                    ans += 1;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example() {
        let source = vec![1, 2, 3, 4];
        let target = vec![2, 1, 4, 5];
        let allowed_swaps = vec![vec![0, 1], vec![2, 3]];
        assert_eq!(
            1,
            Solution::minimum_hamming_distance(source, target, allowed_swaps)
        );

        let source = vec![1, 2, 3, 4];
        let target = vec![1, 3, 2, 4];
        let allowed_swaps = vec![];
        assert_eq!(
            2,
            Solution::minimum_hamming_distance(source, target, allowed_swaps)
        );

        let source = vec![5, 1, 2, 4, 3];
        let target = vec![1, 5, 4, 2, 3];
        let allowed_swaps = vec![vec![0, 4], vec![4, 2], vec![1, 3], vec![1, 4]];
        assert_eq!(
            0,
            Solution::minimum_hamming_distance(source, target, allowed_swaps)
        );
    }

    #[test]
    fn test1() {
        let source = vec![2, 3, 1];
        let target = vec![1, 2, 2];
        let allowed_swaps = vec![vec![0, 2], vec![1, 2]];
        assert_eq!(
            1,
            Solution::minimum_hamming_distance(source, target, allowed_swaps)
        );
    }
}
