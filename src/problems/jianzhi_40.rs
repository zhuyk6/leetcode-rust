#[allow(dead_code)]
pub fn get_least_numbers(mut arr: Vec<i32>, kth: i32) -> Vec<i32> {
    let n = arr.len();

    let mut l = 0;
    let mut r = n;
    let kth = kth as usize;

    let mut steps = 0;
    while l < r {
        let v = arr[(l + r) >> 1];

        println!("step = {}, l = {}, r = {}, v = {}", steps, l, r, v);

        // [l, i) less than v
        // [i, k) equal to v
        // [k, r) greater than v
        let mut i = l;
        let mut j = l;
        let mut k = r;
        while j < k {
            match arr[j].cmp(&v) {
                std::cmp::Ordering::Less => {
                    arr.swap(i, j);
                    i += 1;
                    j += 1;
                }
                std::cmp::Ordering::Equal => {
                    j += 1;
                }
                std::cmp::Ordering::Greater => {
                    k -= 1;
                    arr.swap(j, k);
                }
            }
        }
        println!("arr: {:?}", arr);
        println!("i = {}, j = {}, k = {}", i, j, k);

        if i < kth && kth < k + 1 {
            return arr[..kth].to_vec();
        } else if kth <= i {
            r = i;
        } else {
            l = k;
        }

        steps += 1;
        if steps > 10 {
            panic!("infinite loop")
        }
    }
    arr[..kth].to_vec()
}

#[test]
fn example() {
    let arr = vec![0, 1, 2, 1];
    let k = 1;
    let mut ans = get_least_numbers(arr, k);
    ans.sort();
    assert_eq!(ans, vec![0]);
}

#[test]
fn wrong() {
    let arr = vec![0, 0, 1, 2, 4, 2, 2, 3, 1, 4];
    let k = 8;
    let mut ans = get_least_numbers(arr, k);
    ans.sort();
    assert_eq!(ans, vec![0, 0, 1, 1, 2, 2, 2, 3]);
}

#[test]
fn wrong2() {
    let arr = vec![2, 1, 4];
    let k = 2;
    assert_eq!(get_least_numbers(arr, k), vec![1, 2]);
}
