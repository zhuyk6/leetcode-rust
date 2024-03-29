struct Solution;

const P: i32 = 1_000_000_007;

#[allow(unused)]
impl Solution {
    pub fn first_day_been_in_all_rooms(next_visit: Vec<i32>) -> i32 {
        let n = next_visit.len();

        let mut first_visit: Vec<Vec<Option<i32>>> = vec![vec![None, None]; n];
        let mut even_visit: Vec<bool> = vec![true; n];
        let mut visit = vec![false; n];

        let mut now = 0;
        let mut x = 0;
        let mut num_vis = 1;

        first_visit[0][1] = Some(0);
        even_visit[0] = false;
        visit[0] = true;

        loop {
            let px = if even_visit[x] { 0 } else { 1 };
            println!("x = {x}, px = {px}, now = {now}");

            let y = if even_visit[x] {
                (x + 1) % n
            } else {
                next_visit[x] as usize
            };
            let py = if even_visit[y] { 1 } else { 0 };
            println!("y = {y}, py = {py}");

            println!("f = {:?}", first_visit);

            if let Some(t) = first_visit[y][py] {
                // after d time, return back to x
                let d = (now - t + P) % P;
                now = (now + 1 + d) % P;
                even_visit[x] = !even_visit[x];
                first_visit[x][px ^ 1] = Some(now);
            } else {
                x = y;
                now = (now + 1) % P;
                first_visit[y][py] = Some(now);
                even_visit[y] = !even_visit[y];
                if !visit[y] {
                    visit[y] = true;
                    num_vis += 1;
                }
            }
            if num_vis == n {
                break;
            }
        }

        now
    }
}

#[test]
fn test1() {
    let next_visit = vec![0, 0];
    assert_eq!(Solution::first_day_been_in_all_rooms(next_visit), 2);
}

#[test]
fn test2() {
    let next_visit = vec![0, 0, 2];
    assert_eq!(Solution::first_day_been_in_all_rooms(next_visit), 6);
}

#[test]
fn test3() {
    let next_visit = vec![0, 1, 2, 0];
    assert_eq!(Solution::first_day_been_in_all_rooms(next_visit), 6);
}

#[test]
fn test4() {
    let next_visit = vec![0, 0, 1, 2];
    assert_eq!(Solution::first_day_been_in_all_rooms(next_visit), 12);
}
