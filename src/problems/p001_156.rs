pub fn max_rep_opt1(text: String) -> i32 {
    fn forward(text: String) -> i32 {
        let cnt = {
            let mut cnt = [0; 26];
            for c in text.bytes() {
                cnt[(c - b'a') as usize] += 1;
            }
            cnt
        };

        let s = text.as_bytes();
        let n = s.len();

        let mut ans = 0;
        let mut i = 0;
        while i < n {
            let mut j = i;
            while j + 1 < n && s[j + 1] == s[i] {
                j += 1;
            }

            ans = ans.max(cnt[(s[i] - b'a') as usize].min(j - i + 1));

            println!("i = {i}, j = {j}");

            let next_i = j + 1;
            if next_i >= n {
                break;
            }

            j += 1;
            while j + 1 < n && s[j + 1] == s[i] {
                j += 1;
            }

            ans = ans.max(cnt[(s[i] - b'a') as usize].min(j - i + 1));

            println!("i = {i}, j = {j}");
            i = next_i;
        }
        ans as i32
    }

    let reverse = text.chars().rev().collect();
    forward(text).max(forward(reverse))
}

#[test]
fn example() {
    let text = "ababa".to_string();
    assert_eq!(max_rep_opt1(text), 3);

    let text = "aaabaaa".to_string();
    assert_eq!(max_rep_opt1(text), 6);
}
