pub struct Solution;

impl Solution {
    pub fn clear_stars(s: String) -> String {
        let mut alpha_pos: Vec<Vec<usize>> = vec![vec![]; 26];
        let s = s.as_bytes();
        let mut deleted = vec![false; s.len()];

        for (i, &c) in s.iter().enumerate() {
            if c.is_ascii_lowercase() {
                alpha_pos[(c - b'a') as usize].push(i);
            } else {
                deleted[i] = true;
                for positions in alpha_pos.iter_mut() {
                    if let Some(pos) = positions.pop() {
                        deleted[pos] = true;
                        break;
                    }
                }
            }
        }

        let bytes: Vec<u8> = s
            .iter()
            .enumerate()
            .filter_map(|(i, &c)| if !deleted[i] { Some(c) } else { None })
            .collect();

        String::from_utf8(bytes).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let s = "aaba*".to_string();
        assert_eq!(Solution::clear_stars(s), "aab".to_string());
    }

    #[test]
    fn sample2() {
        let s = "abc".to_string();
        assert_eq!(Solution::clear_stars(s), "abc".to_string());
    }

    #[test]
    fn issue() {
        let s = "d*o*".to_string();
        assert_eq!(Solution::clear_stars(s), "".to_string());
    }
}
