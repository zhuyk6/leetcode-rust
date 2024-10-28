use std::collections::HashSet;

/// z = x^i + y^j, where i >= 0 and j >= 0
pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
    if x > y {
        return powerful_integers(y, x, bound);
    }
    if x == 1 {
        if y == 1 {
            if 2 <= bound {
                vec![2]
            } else {
                vec![]
            }
        } else {
            let mut v = HashSet::new();
            let mut acc_y = 1;
            while acc_y < bound {
                v.insert(1 + acc_y);
                acc_y *= y;
            }
            v.into_iter().collect()
        }
    } else {
        let mut v = HashSet::new();
        let mut acc_x = 1;
        while acc_x < bound {
            let mut acc_y = 1;
            while acc_x + acc_y <= bound {
                v.insert(acc_x + acc_y);
                acc_y *= y;
            }
            acc_x *= x;
        }
        v.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let x = 2;
        let y = 3;
        let bound = 10;
        let mut v = powerful_integers(x, y, bound);
        v.sort();
        assert_eq!(v, vec![2, 3, 4, 5, 7, 9, 10]);
    }
}
