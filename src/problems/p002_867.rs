pub struct Solution;

// fn is_prime(n: usize) -> bool {
//     assert!(n >= 1);
//     if n == 1 {
//         return false;
//     }
//     for p in 2..n {
//         if p * p > n {
//             break;
//         }
//         if n % p == 0 {
//             return false;
//         }
//     }
//     true
// }

fn generate_prime(n: usize) -> Vec<bool> {
    let mut not_prime = vec![false; n + 1];
    not_prime[0] = true;
    not_prime[1] = true;
    let mut primes = vec![];
    for i in 2..=n {
        if !not_prime[i] {
            primes.push(i);
        }
        for &j in primes.iter() {
            if j * i > n {
                break;
            }
            not_prime[j * i] = true;
            if i % j == 0 {
                break;
            }
        }
    }
    println!("{primes:#?}");
    println!("{not_prime:#?}");
    not_prime
}

#[allow(clippy::upper_case_acronyms)]
struct DFS<'a> {
    not_prime: &'a Vec<bool>,
    to: &'a Vec<Vec<usize>>,
    f0: Vec<usize>,
    f1: Vec<usize>,
    ans: usize,
}

impl<'a> DFS<'a> {
    fn new(n: usize, not_prime: &'a Vec<bool>, to: &'a Vec<Vec<usize>>) -> Self {
        DFS {
            not_prime,
            to,
            f0: vec![0; n],
            f1: vec![0; n],
            ans: 0,
        }
    }
    fn dfs(&mut self, x: usize, fa: usize) {
        let mut f0 = 0;
        let mut f1 = 0;
        for &y in self.to[x].iter() {
            if y != fa {
                self.dfs(y, x);
                f0 += self.f0[y];
                f1 += self.f1[y];
            }
        }
        if !self.not_prime[x] {
            self.f0[x] = 0;
            self.f1[x] = f0 + 1;

            self.ans += f0;
            let mut tmp = 0;
            for &y in self.to[x].iter() {
                if y != fa {
                    tmp += self.f0[y] * (f0 - self.f0[y]);
                }
            }
            self.ans += tmp / 2;
        } else {
            self.f0[x] = f0 + 1;
            self.f1[x] = f1;

            self.ans += f1;
            let mut tmp = 0;
            for &y in self.to[x].iter() {
                if y != fa {
                    tmp += self.f1[y] * (f0 - self.f0[y]);
                }
            }
            self.ans += tmp;
            println!("tmp = {tmp}");
        }
        println!(
            "x = {}, fx0 = {}, fx1 = {}, f0 = {}, f1 = {}, ans = {}",
            x, self.f0[x], self.f1[x], f0, f1, self.ans
        );
    }
}

impl Solution {
    pub fn count_paths(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut to: Vec<Vec<usize>> = vec![vec![]; n + 1];
        for e in edges.into_iter() {
            to[e[0] as usize].push(e[1] as usize);
            to[e[1] as usize].push(e[0] as usize);
        }
        let not_prime = generate_prime(n);

        let mut dfs = DFS::new(n + 1, &not_prime, &to);
        dfs.dfs(1, 0);
        dfs.ans as i64
    }
}

#[test]
fn test1() {
    let n = 5;
    let edges = vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![2, 5]];

    let ans = Solution::count_paths(n, edges);
    assert_eq!(ans, 4);
}

#[test]
fn test2() {
    let n = 6;
    let edges = vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![3, 5], vec![3, 6]];

    let ans = Solution::count_paths(n, edges);
    assert_eq!(ans, 6);
}

#[test]
fn test3() {
    let n = 9;
    let edges = vec![
        vec![7, 4],
        vec![3, 4],
        vec![5, 4],
        vec![1, 5],
        vec![6, 4],
        vec![9, 5],
        vec![8, 7],
        vec![2, 8],
    ];

    let ans = Solution::count_paths(n, edges);
    assert_eq!(ans, 17);
}
