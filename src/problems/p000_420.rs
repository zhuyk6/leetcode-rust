pub struct Solution;

#[inline]
fn count1(s: &[u8]) -> usize {
    s.iter().filter(|c| c.is_ascii_lowercase()).count()
}

#[inline]
fn count2(s: &[u8]) -> usize {
    s.iter().filter(|c| c.is_ascii_uppercase()).count()
}

#[inline]
fn count3(s: &[u8]) -> usize {
    s.iter().filter(|c| c.is_ascii_digit()).count()
}

#[inline]
fn get_continuous(s: &[u8], l: usize) -> usize {
    let mut r = l + 1;
    let n = s.len();
    while r < n && s[l] == s[r] {
        r += 1;
    }
    r
}

fn add_characters(s: &mut Vec<u8>) -> u32 {
    fn get_cha(s: &[u8], c: u8) -> u8 {
        if count1(s) == 0 {
            b'a'
        } else if count2(s) == 0 {
            b'A'
        } else if count3(s) == 0 {
            b'0'
        } else if c.is_ascii_alphabetic() {
            b'9'
        } else {
            b'z'
        }
    }

    let mut ops = 0;
    let mut i = 0;
    while s.len() < 6 && i < s.len() {
        let j = get_continuous(s, i);
        if j - i >= 3 {
            // insert at (i+2)
            let c = get_cha(s, s[i]);
            s.insert(i + 2, c);
            ops += 1;
            i += 3;
        } else {
            i = j;
        }
    }
    while s.len() < 6 {
        let c = get_cha(s, *s.last().unwrap());
        s.push(c);
        ops += 1;
    }
    ops
}

fn del_characters(s: &mut Vec<u8>) -> u32 {
    let mut ops = 0;

    // delete continuous
    let mut stack = vec![];
    loop {
        stack.clear();

        let mut i = 0;
        while i < s.len() {
            let j = get_continuous(s, i);
            if j - i >= 3 {
                stack.push((i, j, (j - i) % 3));
            }
            i = j;
        }

        if stack.is_empty() {
            break;
        }

        stack.sort_by_key(|(_, _, v)| *v);
        let pos = stack[0].0;
        let mut cnt = stack[0].2 + 1;
        while s.len() > 20 && cnt > 0 {
            s.remove(pos);
            ops += 1;
            cnt -= 1;
        }
        if s.len() == 20 {
            break;
        }
    }

    while s.len() > 20 {
        if count1(s) > 1 {
            // del lower
            let pos = s.iter().position(|c| c.is_ascii_lowercase()).unwrap();
            s.remove(pos);
            ops += 1;
        } else if count2(s) > 1 {
            // del upper
            let pos = s.iter().position(|c| c.is_ascii_uppercase()).unwrap();
            s.remove(pos);
            ops += 1;
        } else if count3(s) > 1 {
            // del digit
            let pos = s.iter().position(|c| c.is_ascii_digit()).unwrap();
            s.remove(pos);
            ops += 1;
        } else {
            // del last
            s.pop();
        }
    }

    ops
}

fn fix_characters(s: &mut [u8]) -> u32 {
    let n = s.len();

    let mut ops = 0;

    let fs = [count1, count2, count3];
    let gs = [
        u8::is_ascii_lowercase,
        u8::is_ascii_uppercase,
        u8::is_ascii_digit,
    ];
    let cs = [b'a', b'A', b'0'];

    // continuous >= 3
    let mut i = 0;
    while i < n {
        let j = get_continuous(s, i);
        if j - i >= 3 {
            // change (i+2)
            let c = {
                let mut c = b' ';
                for k in 0..3 {
                    if fs[k](s) == 0 {
                        c = cs[k];
                        break;
                    }
                }
                if c == b' ' {
                    if s[i].is_ascii_alphabetic() {
                        c = b'9';
                    } else {
                        c = b'z';
                    }
                }
                c
            };
            s[i + 2] = c;
            ops += 1;
            i += 3;
        } else {
            i = j;
        }
    }

    for i in 0..3 {
        if fs[i](s) == 0 {
            for j in 0..3 {
                if fs[j](s) > 1 {
                    let pos = s.iter().position(gs[j]).unwrap();
                    s[pos] = cs[i];
                    ops += 1;
                }
            }
        }
        if fs[i](s) == 0 {
            let pos = s.iter().position(|c| !c.is_ascii_alphanumeric()).unwrap();
            s[pos] = cs[i];
            ops += 1;
        }
    }

    ops
}

impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        let mut s: Vec<u8> = password.as_bytes().to_vec();

        let mut ops = 0;

        // add
        if s.len() < 6 {
            ops += add_characters(&mut s);
        }
        println!("After Add: {:?}, ops = {ops}", String::from_utf8_lossy(&s));

        // del
        if s.len() > 20 {
            ops += del_characters(&mut s);
        }
        println!("After Del: {:?}, ops = {ops}", String::from_utf8_lossy(&s));

        // fix
        ops += fix_characters(&mut s);
        println!("After Fix: {:?}, ops = {ops}", String::from_utf8_lossy(&s));

        ops as i32
    }
}

#[test]
fn test1() {
    let s = "a".to_string();
    assert_eq!(Solution::strong_password_checker(s), 5);
}

#[test]
fn test2() {
    let s = "aA1".to_string();
    assert_eq!(Solution::strong_password_checker(s), 3);
}

#[test]
fn test3() {
    let s = "1337C0d3".to_string();
    assert_eq!(Solution::strong_password_checker(s), 0);
}

#[test]
fn test4() {
    let s = "bbaaaaaaaaaaaaaaacccccc".to_string();
    assert_eq!(Solution::strong_password_checker(s), 8);
}
