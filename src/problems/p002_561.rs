pub struct Solution;

impl Solution {
    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
        assert!(basket1.len() == basket2.len());

        let mut a = basket1;
        let mut b = basket2;
        a.sort_unstable();
        b.sort_unstable();

        let min_val = a[0].min(b[0]);

        fn group(arr: Vec<i32>) -> Vec<(i32, u32)> {
            let mut g = vec![];
            let mut cur = i32::MIN;
            let mut cnt = 0;
            for v in arr {
                if v != cur {
                    if cnt > 0 {
                        g.push((cur, cnt));
                    }
                    cur = v;
                    cnt = 1;
                } else {
                    cnt += 1;
                }
            }
            if cnt > 0 {
                g.push((cur, cnt));
            }

            g
        }

        let g1 = group(a);
        let g2 = group(b);

        // dbg!(&g1);
        // dbg!(&g2);

        let mut trans1: Vec<(i32, u32)> = vec![];
        let mut trans2: Vec<(i32, u32)> = vec![];

        let mut i = 0;
        let mut j = 0;
        while i < g1.len() && j < g2.len() {
            match g1[i].0.cmp(&g2[j].0) {
                std::cmp::Ordering::Less => {
                    if g1[i].1 % 2 != 0 {
                        return -1;
                    }
                    trans1.push((g1[i].0, g1[i].1 / 2));
                    i += 1;
                }
                std::cmp::Ordering::Greater => {
                    if g2[j].1 % 2 != 0 {
                        return -1;
                    }
                    trans2.push((g2[j].0, g2[j].1 / 2));
                    j += 1;
                }
                std::cmp::Ordering::Equal => {
                    let cnt = g1[i].1 + g2[j].1;
                    if cnt % 2 != 0 {
                        return -1;
                    }
                    let half = cnt / 2;
                    if g1[i].1 > half {
                        trans1.push((g1[i].0, g1[i].1 - half));
                    } else if g2[j].1 > half {
                        trans2.push((g2[j].0, g2[j].1 - half));
                    }
                    i += 1;
                    j += 1;
                }
            }
        }
        while i < g1.len() {
            if g1[i].1 % 2 != 0 {
                return -1;
            }
            trans1.push((g1[i].0, g1[i].1 / 2));
            i += 1;
        }
        while j < g2.len() {
            if g2[j].1 % 2 != 0 {
                return -1;
            }
            trans2.push((g2[j].0, g2[j].1 / 2));
            j += 1;
        }

        // dbg!(&trans1);
        // dbg!(&trans2);

        let mut a = vec![];
        for (v, c) in trans1 {
            a.extend(std::iter::repeat_n(v, c as usize));
        }
        let mut b = vec![];
        for (v, c) in trans2 {
            b.extend(std::iter::repeat_n(v, c as usize));
        }
        let mut ans = 0_i64;

        for (v1, v2) in a.into_iter().zip(b.into_iter().rev()) {
            let cost1 = v1.min(v2) as i64;
            let cost2 = 2 * min_val as i64;
            ans += cost1.min(cost2);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let basket1 = vec![4, 2, 2, 2];
        let basket2 = vec![1, 4, 1, 2];
        assert_eq!(Solution::min_cost(basket1, basket2), 1);
    }

    #[test]
    fn sample2() {
        let basket1 = vec![2, 3, 4, 1];
        let basket2 = vec![3, 2, 5, 1];
        assert_eq!(Solution::min_cost(basket1, basket2), -1);
    }

    #[test]
    fn issue1() {
        let basket1 = vec![84, 80, 43, 8, 80, 88, 43, 14, 100, 88];
        let basket2 = vec![32, 32, 42, 68, 68, 100, 42, 84, 14, 8];
        assert_eq!(Solution::min_cost(basket1, basket2), 48);
    }

    #[test]
    fn issue2() {
        let basket1 = vec![4, 4, 4, 4, 3];
        let basket2 = vec![5, 5, 5, 5, 3];
        assert_eq!(Solution::min_cost(basket1, basket2), 8);
    }
}
