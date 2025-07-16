pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn build_bridge_force(num: i32, wood: Vec<Vec<i32>>) -> i64 {
        let n = wood.len();
        let num = num as usize;

        let lengths: Vec<usize> = wood.iter().map(|w| (w[1] - w[0]) as usize).collect();

        let mut dp = vec![vec![i64::MAX; num + 1]; n];

        // the first wood
        for l in 1..=num {
            let r = l + lengths[0];
            if r > num {
                break;
            }
            dp[0][l] = (wood[0][0] as i64 - l as i64).abs();
        }

        // fill the dp table
        for i in 1..n {
            for l in 1..=num {
                let r = l + lengths[i];
                if r > num {
                    break;
                }
                let cost = (wood[i][0] as i64 - l as i64).abs();

                for ll in 1..=num {
                    let rr = ll + lengths[i - 1];
                    if rr > num {
                        break;
                    }
                    if dp[i - 1][ll] == i64::MAX || rr < l || r < ll {
                        continue;
                    }

                    dp[i][l] = dp[i][l].min(dp[i - 1][ll] + cost);
                }
            }
        }

        dp[n - 1].iter().copied().min().unwrap_or(i64::MAX)
    }

    #[allow(dead_code)]
    pub fn build_bridge_monotonic_queue(num: i32, wood: Vec<Vec<i32>>) -> i64 {
        let n = wood.len();
        let num = num as usize;

        let lengths: Vec<usize> = wood.iter().map(|w| (w[1] - w[0]) as usize).collect();

        let mut dp = vec![vec![i64::MAX; num + 1]; n];

        // the first wood
        for l in 1..=num {
            let r = l + lengths[0];
            if r > num {
                break;
            }
            dp[0][l] = (wood[0][0] as i64 - l as i64).abs();
        }

        // fill the dp table
        for i in 1..n {
            // Maintain a monotonic queue to find the minimum dp[i - 1][ll]
            use std::collections::VecDeque;
            let mut queue: VecDeque<(usize, i64)> = VecDeque::new();

            assert!(lengths[i] < num);
            let cost = (wood[i][0] as i64 - 1_i64).abs();
            for ll in 1..=1 + lengths[i] {
                while let Some(&(_, last_cost)) = queue.back() {
                    if last_cost >= dp[i - 1][ll] {
                        queue.pop_back();
                    } else {
                        break;
                    }
                }
                queue.push_back((ll, dp[i - 1][ll]));
            }
            dp[i][1] = cost + queue.front().map_or(0, |&(_, c)| c);

            for l in 1..=num {
                let r = l + lengths[i];
                if r > num {
                    break;
                }
                let cost = (wood[i][0] as i64 - l as i64).abs();

                // l - lengths[i - 1] <= ll <= l + lengths[i]
                if l > lengths[i - 1] {
                    let ll = l - lengths[i - 1];
                    while let Some(&(last_ll, _)) = queue.front() {
                        if last_ll < ll {
                            queue.pop_front();
                        } else {
                            break;
                        }
                    }
                }
                let ll = l + lengths[i];
                while let Some(&(_, last_cost)) = queue.back() {
                    if last_cost >= dp[i - 1][ll] {
                        queue.pop_back();
                    } else {
                        break;
                    }
                }
                queue.push_back((ll, dp[i - 1][ll]));

                dp[i][l] = cost + queue.front().map_or(0, |&(_, c)| c);
            }
        }

        dp[n - 1].iter().copied().min().unwrap_or(i64::MAX)
    }

    #[allow(dead_code)]
    pub fn build_bridge(num: i32, wood: Vec<Vec<i32>>) -> i64 {
        let n = wood.len();
        let num = num as i64;

        let lengths: Vec<i64> = wood.iter().map(|w| (w[1] - w[0]) as i64).collect();

        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut f1: BinaryHeap<i64> = Default::default();
        let mut f2: BinaryHeap<Reverse<i64>> = Default::default();

        let mut min_cost = 0; // count the minimum value of f

        // maintain f2, the segment [min_left, min_right] which are minimum points of f
        let mut min_left = wood[0][0] as i64;
        let mut min_right = wood[0][0] as i64;

        // maintain f1 and f3
        f1.push(wood[0][0] as i64);
        f2.push(Reverse(wood[0][0] as i64));
        let mut delta1 = 0_i64;
        let mut delta2 = 0_i64;

        for i in 1..n {
            let li = wood[i][0] as i64;

            // calculate g
            // g1(l) = f(l + lengths[i])
            // g3(l) = f(l - lengths[i - 1])
            min_left -= lengths[i];
            min_right += lengths[i - 1];
            delta1 -= lengths[i];
            delta2 += lengths[i - 1];

            // calculate new f and update min_cost
            // three cases
            if li < min_left {
                // li in (-inf, min_left)

                // update min_cost, the minimum point is at min_left
                assert!(min_left >= 1 && min_left <= num);
                min_cost += min_left - li;

                // update f2
                f2.push(Reverse(min_left - delta2));
                min_right = min_left;

                // update f1
                f1.pop();
                f1.push(li - delta1);
                f1.push(li - delta1);
                min_left = f1.peek().unwrap() + delta1;
            } else if li > min_right {
                // li in the (min_right, inf)

                // update min_cost, the minimum point is at min_right
                assert!(min_right >= 1 && min_right <= num);
                min_cost += li - min_right;

                // update f1
                f1.push(min_right - delta1);
                min_left = min_right;

                // update f2
                f2.pop();
                f2.push(Reverse(li - delta2));
                f2.push(Reverse(li - delta2));
                min_right = f2.peek().unwrap().0 + delta2;
            } else {
                // li in the [min_left, min_right]

                // update f1
                f1.push(li - delta1);
                min_left = li;

                // update f2
                f2.push(Reverse(li - delta2));
                min_right = li;
            }
        }

        min_cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let num = 10;
        let wood = nested_vec![[1, 2], [4, 7], [8, 9]];
        assert_eq!(Solution::build_bridge_force(num, wood.clone()), 3);
        assert_eq!(Solution::build_bridge_monotonic_queue(num, wood.clone()), 3);
        assert_eq!(Solution::build_bridge(num, wood.clone()), 3);
    }

    #[test]
    fn sample2() {
        let num = 10;
        let wood = nested_vec![[1, 5], [1, 1], [10, 10], [6, 7], [7, 8]];
        assert_eq!(Solution::build_bridge_force(num, wood.clone()), 10);
        assert_eq!(
            Solution::build_bridge_monotonic_queue(num, wood.clone()),
            10
        );
        assert_eq!(Solution::build_bridge(num, wood.clone()), 10);
    }

    #[test]
    fn sample3() {
        let num = 5;
        let wood = nested_vec![[1, 2], [2, 4]];
        assert_eq!(Solution::build_bridge_force(num, wood.clone()), 0);
        assert_eq!(Solution::build_bridge_monotonic_queue(num, wood.clone()), 0);
        assert_eq!(Solution::build_bridge(num, wood.clone()), 0);
    }
}
