pub struct Solution;

struct Solver {
    mem: Vec<Option<bool>>,
}

impl Solver {
    fn new(n: usize) -> Self {
        let mut mem = vec![None; n + 1];
        mem[0] = Some(false);
        Solver { mem }
    }

    fn dfs(&mut self, n: usize) -> bool {
        if let Some(result) = self.mem[n] {
            return result;
        }
        for i in 1..=n {
            let square = i * i;
            if square > n {
                break;
            }
            if !self.dfs(n - square) {
                self.mem[n] = Some(true);
                return true;
            }
        }
        self.mem[n] = Some(false);
        false
    }
}

impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let mut solver = Solver::new(n as usize);
        solver.dfs(n as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let n = 1;
        assert!(Solution::winner_square_game(n));
    }

    #[test]
    fn sample2() {
        let n = 2;
        assert!(!Solution::winner_square_game(n));
    }

    #[test]
    fn sample3() {
        let n = 4;
        assert!(Solution::winner_square_game(n));
    }

    #[test]
    fn sample4() {
        let n = 7;
        assert!(!Solution::winner_square_game(n));
    }

    #[test]
    fn sample5() {
        let n = 17;
        assert!(!Solution::winner_square_game(n));
    }
}
