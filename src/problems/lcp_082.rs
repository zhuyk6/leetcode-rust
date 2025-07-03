pub struct Solution;

use std::{cell::RefCell, collections::HashMap};

pub mod brute_force {
    pub struct Solution;

    use super::{mul, pow};
    use std::collections::HashMap;

    struct Dfs {
        arr: Vec<i32>,
        arr_len: Vec<usize>,
        p: i32,
        mem: Vec<Option<Vec<(i32, usize, u32)>>>,
    }

    impl Dfs {
        fn new(arr: Vec<i32>, p: i32) -> Self {
            let arr_len = arr
                .iter()
                .map(|&x| match x {
                    0 => 1,
                    _ => {
                        let mut x = x;
                        let mut len = 0;
                        while x > 0 {
                            len += 1;
                            x /= 10;
                        }
                        len
                    }
                })
                .collect();

            let n = arr.len();
            Self {
                arr,
                arr_len,
                p,
                mem: vec![None; 1 << n],
            }
        }

        #[inline]
        fn get_pos(mask: usize) -> usize {
            match mask {
                0b000000001 => 0,
                0b000000010 => 1,
                0b000000100 => 2,
                0b000001000 => 3,
                0b000010000 => 4,
                0b000100000 => 5,
                0b001000000 => 6,
                0b010000000 => 7,
                0b100000000 => 8,
                _ => panic!("Invalid mask"),
            }
        }

        /// (ans, len) -- cnt
        fn dfs(&mut self, mask: usize) {
            // if already calculated
            if self.mem[mask].is_some() {
                return;
            }

            // corner case
            if mask.count_ones() == 1 {
                // 1{}9
                let pos = Dfs::get_pos(mask);
                let val = self.arr[pos];
                let len = self.arr_len[pos];
                let mut ans = (mul(val, 10, self.p) + 9) % self.p;
                ans = (ans + pow(10, len + 1, self.p)) % self.p;

                let v = vec![(ans, self.arr_len[pos] + 2, 1)];
                self.mem[mask] = Some(v);
                return;
            }
            // iterate all subsets of mask
            let mut s = mask;
            let mut map: HashMap<(i32, usize), u32> = HashMap::new();
            while s > 0 {
                s = (s - 1) & mask;
                if s == 0 {
                    break;
                }
                self.dfs(s);
                self.dfs(mask - s);
                let left = self.mem[s].as_ref().unwrap();
                let right = self.mem[mask - s].as_ref().unwrap();
                for &(l, len_l, cnt_l) in left {
                    for &(r, len_r, cnt_r) in right {
                        // 1{left}{right}9
                        let mut ans = pow(10, len_l + len_r + 1, self.p);
                        ans = (ans + mul(l, pow(10, len_r + 1, self.p), self.p)) % self.p;
                        ans = (ans + mul(r, 10, self.p)) % self.p;
                        ans = (ans + 9) % self.p;

                        *map.entry((ans, len_l + len_r + 2)).or_default() += cnt_l * cnt_r;
                    }
                }
            }
            self.mem[mask] = Some(map.into_iter().map(|(a, b)| (a.0, a.1, b)).collect());
        }
    }

    impl Solution {
        pub fn tree_of_infinite_souls(gem: Vec<i32>, p: i32, target: i32) -> i32 {
            let n = gem.len();
            let mask = (1 << n) - 1;
            let mut sol = Dfs::new(gem, p);
            sol.dfs(mask);

            // let total_len = sol.arr_len.iter().sum::<usize>() + (2 * n - 1) * 2;
            match sol.mem[mask]
                .as_ref()
                .unwrap()
                .iter()
                .find(|&&(ans, _, _)| ans == target)
            {
                Some((_, _, cnt)) => *cnt as i32,
                None => 0,
            }
        }
    }
}

pub mod method2 {
    use super::{HashMap, get_len, mul};

    pub fn generate_sequence_of_entries(n: usize) -> Vec<Vec<Option<i32>>> {
        fn dfs(n: usize, mem: &mut Vec<Vec<String>>) {
            if !mem[n].is_empty() {
                return;
            }

            if n == 1 {
                let s = "1?9".to_string();
                mem[1].push(s);
            } else {
                let mut v = vec![];
                for i in 1..n {
                    dfs(i, mem);
                    dfs(n - i, mem);
                    for left in &mem[i] {
                        for right in &mem[n - i] {
                            let s = format!("1{left}{right}9");
                            v.push(s);
                        }
                    }
                }
                mem[n] = v;
            }
        }

        fn parse(s: &str) -> Vec<Option<i32>> {
            let mut v = vec![];

            let mut val = 0;
            for c in s.bytes() {
                match c {
                    b'1' | b'9' => val = val * 10 + (c - b'0') as i32,
                    b'?' => {
                        v.push(Some(val));
                        v.push(None);
                        val = 0;
                    }
                    _ => panic!("Invalid character"),
                }
            }
            if val > 0 {
                v.push(Some(val));
            }

            v
        }

        let mut mem: Vec<Vec<String>> = vec![vec![]; n + 1];
        dfs(n, &mut mem);
        mem[n].iter().map(|s| parse(s)).collect()
    }

    pub fn generate_permutation(
        n: usize,
        m: usize,
        used: &mut [bool],
        seq: &mut Vec<usize>,
        ans: &mut Vec<Vec<usize>>,
    ) {
        if m == 0 {
            ans.push(seq.clone());
            return;
        }

        for i in 0..n {
            if !used[i] {
                used[i] = true;
                seq.push(i);
                generate_permutation(n, m - 1, used, seq, ans);
                used[i] = false;
                seq.pop();
            }
        }
    }

    pub fn generate_choices(n: usize, m: usize) -> Vec<Vec<usize>> {
        fn dfs(x: usize, n: usize, m: usize, choosed: &mut Vec<usize>, ans: &mut Vec<Vec<usize>>) {
            if m == 0 {
                ans.push(choosed.clone());
                return;
            }
            if x == n || n - x < m {
                return;
            }
            // not choose x
            dfs(x + 1, n, m, choosed, ans);
            // choose x
            choosed.push(x);
            dfs(x + 1, n, m - 1, choosed, ans);
            choosed.pop();
        }

        let mut choosed = Vec::with_capacity(m);
        let mut ans = vec![];
        dfs(0, n, m, &mut choosed, &mut ans);
        ans
    }

    pub const MAX_LEN: usize = 200;

    pub struct Solver {
        pow10: [i32; MAX_LEN],
        perms: HashMap<usize, Vec<Vec<usize>>>,
    }

    impl Solver {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self {
                pow10: [0; MAX_LEN],
                perms: HashMap::new(),
            }
        }

        pub fn calc(
            &self,
            seq: &[Option<i32>],
            choosed: &[usize],
            arr: &[i32],
            p: i32,
            init_val: i32,
        ) -> HashMap<i32, u32> {
            let n = choosed.len();
            let perms = self.perms[&n].as_slice();

            let mut map: HashMap<i32, u32> = HashMap::new();
            for perm in perms {
                let mut idx = 0;
                let mut val = init_val;
                for entry in seq {
                    match entry {
                        None => {
                            // let v = arr[choosed[perm[idx]]];
                            let v = unsafe {
                                *arr.get_unchecked(*choosed.get_unchecked(*perm.get_unchecked(idx)))
                            };
                            val = (mul(val, self.pow10[get_len(v)], p) + v) % p;
                            idx += 1;
                        }
                        Some(v) => {
                            val = (mul(val, self.pow10[get_len(*v)], p) + v) % p;
                        }
                    }
                }
                *map.entry(val).or_default() += 1;
            }
            map
        }

        pub fn solve(&mut self, mut gem: Vec<i32>, p: i32, target: i32) -> i32 {
            let n = gem.len();

            // calculate POW10
            self.pow10[0] = 1;
            for i in 1..200 {
                self.pow10[i] = mul(self.pow10[i - 1], 10, p);
            }

            let seq = generate_sequence_of_entries(n);

            // generate choices
            // if even, then choose from [0,n], if odd from [0, n-1]
            let choices = if n % 2 == 0 {
                generate_choices(n, n / 2)
            } else {
                generate_choices(n - 1, n / 2)
            };

            // generate permutations
            self.perms.insert(n / 2, {
                let num = n / 2;
                let mut used = vec![false; num];
                let mut ans = vec![];
                let mut seq = Vec::with_capacity(num);
                generate_permutation(num, num, &mut used, &mut seq, &mut ans);
                ans
            });

            let mut ans = 0;
            // odd case
            if n % 2 == 1 {
                for i in 0..n {
                    // select gem[i] as the first one, move it to the last
                    gem.swap(n - 1, i);

                    // calculate
                    for s in &seq {
                        let val1 = {
                            let mut val = 0;
                            for e in &s[..2] {
                                match e {
                                    None => {
                                        let v = gem[n - 1];
                                        val = (mul(val, self.pow10[get_len(v)], p) + v) % p;
                                    }
                                    Some(v) => {
                                        val = (mul(val, self.pow10[get_len(*v)], p) + v) % p;
                                    }
                                }
                            }
                            val
                        };

                        let len_1: usize = s[n + 1..]
                            .iter()
                            .map(|e| e.as_ref().map(|v| get_len(*v)).unwrap_or_default())
                            .sum();

                        for choosed in &choices {
                            let left = self.calc(&s[2..n + 1], choosed, &gem[..n - 1], p, val1);

                            let choosed_right = (0..(n - 1))
                                .filter(|&j| !choosed.contains(&j))
                                .collect::<Vec<_>>();
                            let len_right: usize = len_1
                                + choosed_right
                                    .iter()
                                    .map(|c| get_len(gem[*c]))
                                    .sum::<usize>();

                            let right = self.calc(&s[n + 1..], &choosed_right, &gem[..n - 1], p, 0);

                            for (l, cnt_l) in left {
                                // {l}{r} = target
                                let tmp = mul(l, self.pow10[len_right], p);
                                let r = (target + p - tmp) % p;
                                ans += right.get(&r).map(|cnt_r| cnt_r * cnt_l).unwrap_or_default();
                            }
                        }
                    }

                    // recover the original order
                    gem.swap(n - 1, i);
                }
            } else {
                // even case
                for s in &seq {
                    let len_1: usize = s[n..]
                        .iter()
                        .map(|e| e.as_ref().map(|v| get_len(*v)).unwrap_or_default())
                        .sum();

                    for choosed in &choices {
                        let left = self.calc(&s[..n], choosed, &gem, p, 0);

                        let choosed_right = (0..n)
                            .filter(|&i| !choosed.contains(&i))
                            .collect::<Vec<_>>();
                        let len_right: usize = len_1
                            + choosed_right
                                .iter()
                                .map(|c| get_len(gem[*c]))
                                .sum::<usize>();

                        let right = self.calc(&s[n..], &choosed_right, &gem, p, 0);

                        for (l, cnt_l) in left {
                            // {l}{r} = target
                            let tmp = mul(l, self.pow10[len_right], p);
                            let r = (target + p - tmp) % p;
                            ans += right.get(&r).map(|cnt_r| cnt_r * cnt_l).unwrap_or_default();
                        }
                    }
                }
            }

            ans as i32
        }
    }
}

#[inline]
fn mul(a: i32, b: i32, p: i32) -> i32 {
    let a = a as i64;
    let b = b as i64;
    let p = p as i64;
    (a * b % p) as i32
}

#[inline]
fn pow(a: i32, n: usize, p: i32) -> i32 {
    let mut a = a % p;
    let mut n = n;
    let mut res = 1;
    while n > 0 {
        if n & 1 == 1 {
            res = mul(res, a, p);
        }
        a = mul(a, a, p);
        n >>= 1;
    }
    res
}

#[inline]
fn inv(a: i32, p: i32) -> Option<i32> {
    match a % p {
        0 => None,
        _ => Some(pow(a, (p - 2) as usize, p)),
    }
}

#[inline]
pub fn get_len(v: i32) -> usize {
    match v {
        0..=9 => 1,
        10..=99 => 2,
        100..=999 => 3,
        1000..=9999 => 4,
        10000..=99999 => 5,
        100000..=999999 => 6,
        1000000..=9999999 => 7,
        10000000..=99999999 => 8,
        100000000..=999999999 => 9,
        _ => panic!("Invalid number"),
    }
}

struct Solver {
    p: i32,
    pow10: [i32; 200],
    mem: Vec<RefCell<HashMap<i32, u32>>>,
    mask_len: Vec<usize>,
}

impl Solver {
    fn new(arr: Vec<i32>, p: i32) -> Self {
        let mut pow10 = [0i32; 200];
        pow10[0] = 1;
        for i in 1..200 {
            pow10[i] = mul(pow10[i - 1], 10, p);
        }

        let n = arr.len();

        let mut mask_len = vec![0; 1 << n];
        for i in 0..n {
            mask_len[1 << i] = get_len(arr[i]) + 2;
        }
        for mask in 1usize..(1 << n) {
            if mask.count_ones() <= 1 {
                continue;
            }
            let s = mask & (!mask + 1); // lowbit of mask
            mask_len[mask] = mask_len[s] + mask_len[mask - s] + 2;
        }

        let mem = vec![RefCell::new(HashMap::new()); 1 << n];
        for i in 0..n {
            let mask = 1 << i;
            // 1{}9
            let mut val = pow10[1 + get_len(arr[i])];
            val = (val + mul(arr[i], 10, p)) % p;
            val = (val + 9) % p;
            mem[mask].borrow_mut().insert(val, 1);
        }

        for mask in 1usize..(1 << n) {
            if !(2..=6).contains(&mask.count_ones()) {
                continue;
            }
            let mut s = mask;
            while s > 0 {
                s = (s - 1) & mask;
                if s == 0 {
                    break;
                }
                let t = mask - s;
                for (&l, &cnt_l) in mem[s].borrow().iter() {
                    for (&r, &cnt_r) in mem[t].borrow().iter() {
                        let len_l = mask_len[s];
                        let len_r = mask_len[t];

                        let mut val = pow10[len_l + len_r + 1];
                        val = (val + mul(l, pow10[len_r + 1], p)) % p;
                        val = (val + mul(r, 10, p)) % p;
                        val = (val + 9) % p;

                        *mem[mask].borrow_mut().entry(val).or_default() += cnt_l * cnt_r;
                    }
                }
            }
        }

        Self {
            p,
            pow10,
            mem,
            mask_len,
        }
    }

    fn dfs(&mut self, mask: usize, target: i32) -> u32 {
        if (1..=6).contains(&mask.count_ones()) {
            return self.mem[mask].borrow().get(&target).copied().unwrap_or(0);
        }
        if self.mem[mask].borrow().contains_key(&target) {
            return self.mem[mask].borrow()[&target];
        }

        let mut ans = 0;
        let mut s = mask;
        while s > 0 {
            s = (s - 1) & mask;
            if s == 0 {
                break;
            }
            let t = mask - s;
            if s.count_ones() <= t.count_ones() {
                let map_s = self.mem[s].borrow().clone();
                for (x, cnt_x) in map_s {
                    let len_x = self.mask_len[s];
                    let len_y = self.mask_len[t];

                    let mut tmp = target;
                    tmp = (tmp + self.p - 9) % self.p;
                    tmp = (tmp + self.p - mul(x, self.pow10[len_y + 1], self.p)) % self.p;
                    tmp = (tmp + self.p - self.pow10[len_x + len_y + 1]) % self.p;

                    // tmp == 10 * y
                    if let Some(inv_10) = inv(10, self.p) {
                        let y = mul(tmp, inv_10, self.p);
                        ans += self.dfs(t, y) * cnt_x;
                    } else if tmp == 0 {
                        // any y is valid
                        for y in 0..self.p {
                            ans += self.dfs(t, y) * cnt_x;
                        }
                    }
                }
            } else {
                let map_t = self.mem[t].borrow().clone();
                for (y, cnt_y) in map_t {
                    let len_x = self.mask_len[s];
                    let len_y = self.mask_len[t];

                    let mut tmp = target;
                    tmp = (tmp + self.p - 9) % self.p;
                    tmp = (tmp + self.p - mul(y, 10, self.p)) % self.p;
                    tmp = (tmp + self.p - self.pow10[len_x + len_y + 1]) % self.p;

                    // tmp == x * 10^(len_y + 1)
                    if let Some(inv_) = inv(self.pow10[len_y + 1], self.p) {
                        let x = mul(tmp, inv_, self.p);
                        ans += self.dfs(s, x) * cnt_y;
                    } else if tmp == 0 {
                        // any x is valid
                        for x in 0..self.p {
                            ans += self.dfs(s, x) * cnt_y;
                        }
                    }
                }
            }
        }
        self.mem[mask].borrow_mut().insert(target, ans);
        ans
    }
}

impl Solution {
    pub fn tree_of_infinite_souls(gem: Vec<i32>, p: i32, target: i32) -> i32 {
        let n = gem.len();
        let mut sol = Solver::new(gem, p);
        sol.dfs((1 << n) - 1, target) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutation() {
        use method2::generate_permutation;
        let n = 2;
        let mut used = vec![false; n];
        let mut seq = Vec::with_capacity(n);
        let mut perms = vec![];
        generate_permutation(2, 2, &mut used, &mut seq, &mut perms);
        assert_eq!(perms, vec![vec![0, 1], vec![1, 0]]);
    }

    #[test]
    fn sample1() {
        let gem = vec![2, 3];
        let p = 100000007;
        let target = 11391299;
        assert_eq!(Solution::tree_of_infinite_souls(gem, p, target), 1);
    }

    #[test]
    fn sample2() {
        let gem = vec![3, 21, 3];
        let p = 7;
        let target = 5;
        assert_eq!(Solution::tree_of_infinite_souls(gem, p, target), 4);
    }

    #[test]
    fn sample3() {
        let gem = vec![1, 2, 3, 4];
        let p = 100000007;
        let target = 30524608;
        assert_eq!(Solution::tree_of_infinite_souls(gem, p, target), 1);
    }

    #[test]
    fn issue() {
        let gem = vec![73];
        let p = 31;
        let target = 3;
        assert_eq!(Solution::tree_of_infinite_souls(gem, p, target), 1);
    }

    #[test]
    fn big_data() {
        let gem = (1..=8).collect();
        let p = 90007;
        let target = 12345;
        assert_eq!(Solution::tree_of_infinite_souls(gem, p, target), 221);
    }

    #[test]
    fn big_data2() {
        let gem = (1..=9).collect();
        let p = 90007;
        let target = 12345;
        assert_eq!(Solution::tree_of_infinite_souls(gem, p, target), 5762);
    }

    #[test]
    fn issue2() {
        let gem = vec![
            321113, 909148, 2108330, 853584, 1000839, 674651, 1585598, 38486, 42347102,
        ];
        let p = 2;
        let target = 1;
        assert_eq!(Solution::tree_of_infinite_souls(gem, p, target), 518918400);
    }
}
