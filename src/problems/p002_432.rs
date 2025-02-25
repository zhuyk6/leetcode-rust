use std::cmp::Reverse;

pub fn hardest_worker<Outer, Inner>(n: i32, logs: Outer) -> i32
where
    Outer: AsRef<[Inner]>,
    Inner: AsRef<[i32]>,
{
    assert!(n >= 2);

    let mut ans = (0, Reverse(0));
    let mut begin = 0;
    for log in logs.as_ref() {
        let id = log.as_ref()[0];
        let end = log.as_ref()[1];
        let time = end - begin;
        begin = end;
        ans = ans.max((time, Reverse(id)));
    }
    ans.1.0
}

#[test]
fn sample() {
    let logs = [[0, 3], [2, 5], [0, 9], [1, 15]];
    let n = 10;
    assert_eq!(hardest_worker(n, logs), 1);
}
