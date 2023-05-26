
pub fn max_nice_divisors(prime_factors: i32) -> i32 {
    #[inline]
    fn mul(a: u32, b: u32) -> u32 {
        const P: u64 = 1000_000_000 + 7;
        let c = (a as u64) * (b as u64);
        (c % P) as u32
    }

    fn pow(mut a: u32, mut n: u32) -> u32 {
        let mut ret = 1;
        while n > 0 {
            if n & 1 > 0 {
                ret = mul(ret, a);
            }
            a = mul(a, a);
            n >>= 1;
        }
        ret
    }

    let n = prime_factors as u32;

    if n <= 4 {
        return n as i32;
    }

    let d = n / 3;
    let ans = match n % 3 {
        0 => pow(3, d),
        1 => mul(pow(3, d-1), 4),
        2 => mul(pow(3, d), 2),
        _ => panic!("impossible!"),
    };
    ans as i32
}

#[test]
fn example() {
    let n = 5;
    assert_eq!(max_nice_divisors(n), 6);

    for i in 0..10 {
        println!("i = {}, f = {}", i, max_nice_divisors(i));
    }
}
