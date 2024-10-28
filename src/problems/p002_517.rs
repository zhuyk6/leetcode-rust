pub fn maximum_tastiness(mut price: Vec<i32>, k: i32) -> i32 {
    price.sort();

    let check = |gap: i32| -> bool {
        let mut cnt = 1;
        let mut last = price[0];
        for v in price.iter().skip(1) {
            if *v - last >= gap {
                last = *v;
                cnt += 1;
            }
        }
        cnt >= k
    };

    let mut l = 0;
    let mut r = price.last().unwrap() - price.first().unwrap();
    let mut ans = 0;
    while l <= r {
        let m = (l + r) >> 1;
        if check(m) {
            ans = m;
            l = m + 1;
        } else {
            r = m - 1;
        }
    }
    ans
}

#[test]
fn example() {
    let price = vec![13, 5, 1, 8, 21, 2];
    let k = 3;
    assert_eq!(maximum_tastiness(price, k), 8);
}
