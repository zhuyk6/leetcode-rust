pub struct Solution;

const MOD: i32 = 1_000_000_007;

#[inline]
fn mul(a: i32, b: i32) -> i32 {
    ((a as i64 * b as i64) % MOD as i64) as i32
}

fn pow(mut a: i32, mut n: u32) -> i32 {
    let mut res = 1;
    while n > 0 {
        if n & 1 == 1 {
            res = mul(res, a);
        }
        a = mul(a, a);
        n >>= 1;
    }
    res
}

impl Solution {
    pub fn get_final_state(mut nums: Vec<i32>, mut k: i32, multiplier: i32) -> Vec<i32> {
        if multiplier == 1 {
            return nums;
        }

        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut heap: BinaryHeap<Reverse<(i32, usize)>> = Default::default();
        for (i, v) in nums.iter().enumerate() {
            heap.push(Reverse((*v, i)));
        }

        let max_v = *nums.iter().max().unwrap();

        while k > 0 {
            if let Some(Reverse((v, id))) = heap.pop() {
                let new_v = v as i64 * multiplier as i64;

                // dbg!(&v, &id);

                if new_v > max_v as i64 {
                    heap.push(Reverse((v, id)));
                    break;
                } else {
                    nums[id] = new_v as i32;
                    heap.push(Reverse((new_v as i32, id)));
                    k -= 1;
                }
            }
        }

        // dbg!(&nums);

        let division = k / nums.len() as i32;
        let mut residue = k % nums.len() as i32;

        // dbg!(&division, &residue);

        for v in nums.iter_mut() {
            *v = mul(*v, pow(multiplier, division as u32));
        }

        // dbg!(&nums);

        while residue > 0 {
            let Reverse((_, id)) = heap.pop().unwrap();
            nums[id] = mul(nums[id], multiplier);
            residue -= 1;
        }

        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![2, 1, 3, 5, 6];
        let k = 5;
        let multiplier = 2;
        assert_eq!(
            Solution::get_final_state(nums, k, multiplier),
            vec![8, 4, 6, 5, 6]
        )
    }

    #[test]
    fn sample2() {
        let nums = vec![100000, 2000];
        let k = 2;
        let multiplier = 1_000_000;
        assert_eq!(
            Solution::get_final_state(nums, k, multiplier),
            vec![999999307, 999999993]
        )
    }
}
