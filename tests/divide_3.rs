#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

fn reverse<T: Clone>(xs: &[T]) -> Vec<T> {
    let mut rev = vec![];
    for x in xs.iter() {
        rev.insert(0, x.clone())
    }
    rev
}

fn divide_3<T: Ord>(arr: &mut [T], l: usize, r: usize, v: &T) -> (usize, usize) {
    // [l, r)
    let mut i = l;
    let mut j = l;
    let mut k = r;
    // [l, i) -- less than v
    // [i, k) -- equal to  v
    // [k, r) -- greater than v
    while j < k {
        match arr[j].cmp(v) {
            std::cmp::Ordering::Less => {
                arr.swap(i, j);
                i += 1;
                j += 1;
            }
            std::cmp::Ordering::Equal => j += 1,
            std::cmp::Ordering::Greater => {
                k -= 1;
                arr.swap(j, k);
            }
        }
    }
    (i, k)
}

#[cfg(test)]
mod tests {
    use quickcheck::{quickcheck, TestResult};

    use super::*;

    #[quickcheck]
    fn prop(xs: Vec<u32>) -> bool {
        xs == reverse(&reverse(&xs))
    }

    #[test]
    fn prop2() {
        fn divide_correct(mut xs: Vec<i32>) -> TestResult {
            let n = xs.len();
            if n == 0 {
                return TestResult::discard();
            }
            let v = xs[n >> 1];
            let (i, j) = divide_3(&mut xs, 0, n, &v);

            TestResult::from_bool(
                xs[..i].iter().all(|x| *x < v)
                    && xs[i..j].iter().all(|x| *x == v)
                    && xs[j..].iter().all(|x| *x > v),
            )
        }
        quickcheck(divide_correct as fn(Vec<i32>) -> TestResult)
    }
}
