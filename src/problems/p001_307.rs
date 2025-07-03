pub struct Solution;

#[derive(Debug)]
struct Dfs {
    rev_words: Vec<Vec<u8>>,
    rev_result: Vec<u8>,
    is_leading: [bool; 26],
    char_to_digit: [Option<i32>; 26],
    used_digit: [bool; 10],
}

impl Dfs {
    fn new(rev_words: Vec<Vec<u8>>, rev_result: Vec<u8>, is_leading: [bool; 26]) -> Self {
        Self {
            rev_words,
            rev_result,
            is_leading,
            char_to_digit: [None; 26],
            used_digit: [false; 10],
        }
    }

    fn assign(&mut self, ch: usize, digit: i32) -> (bool, bool) {
        if let Some(existing_digit) = self.char_to_digit[ch] {
            (existing_digit == digit, false)
        } else if self.used_digit[digit as usize] || (digit == 0 && self.is_leading[ch]) {
            (false, false)
        } else {
            self.char_to_digit[ch] = Some(digit);
            self.used_digit[digit as usize] = true;
            (true, true)
        }
    }

    fn dfs(&mut self, idx: usize, carry: i32, id_word: usize, acc: i32) -> bool {
        if idx == self.rev_result.len() {
            return carry == 0;
        }

        if id_word == self.rev_words.len() {
            let sum = acc + carry;
            let digit = sum % 10;
            let ch = (self.rev_result[idx] - b'A') as usize;

            let (valid, assign) = self.assign(ch, digit);
            if valid && self.dfs(idx + 1, sum / 10, 0, 0) {
                return true;
            }
            if assign {
                self.char_to_digit[ch] = None;
                self.used_digit[digit as usize] = false;
            }
            return false;
        }

        if idx >= self.rev_words[id_word].len() {
            return self.dfs(idx, carry, id_word + 1, acc);
        }

        let ch = (self.rev_words[id_word][idx] - b'A') as usize;
        for digit in 0..10 {
            let (valid, assign) = self.assign(ch, digit);
            if valid {
                let next_acc = acc + digit;
                if self.dfs(idx, carry, id_word + 1, next_acc) {
                    return true;
                }
            }
            if assign {
                self.char_to_digit[ch] = None;
                self.used_digit[digit as usize] = false;
            }
        }

        false
    }
}

impl Solution {
    pub fn is_solvable(words: Vec<String>, result: String) -> bool {
        let is_leading: [bool; 26] = {
            let mut is_leading = [false; 26];
            for word in &words {
                if word.len() > 1
                    && let Some(first_char) = word.chars().next()
                {
                    let idx = first_char as usize - 'A' as usize;
                    is_leading[idx] = true;
                }
            }
            if result.len() > 1
                && let Some(first_char) = result.chars().next()
            {
                let idx = first_char as usize - 'A' as usize;
                is_leading[idx] = true;
            }

            is_leading
        };

        let rev_words = words
            .into_iter()
            .map(|word| word.as_bytes().iter().rev().copied().collect::<Vec<u8>>())
            .collect::<Vec<Vec<u8>>>();
        let rev_result = result.as_bytes().iter().rev().copied().collect::<Vec<u8>>();

        if rev_words.iter().map(|w| w.len()).max().unwrap() > rev_result.len() {
            return false;
        }

        let mut dfs = Dfs::new(rev_words, rev_result, is_leading);

        dfs.dfs(0, 0, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec_owned;

    #[test]
    fn sample1() {
        let words = nested_vec_owned!["SEND", "MORE"];
        let result = "MONEY".to_string();
        assert!(Solution::is_solvable(words, result));
    }

    #[test]
    fn sample2() {
        let words = nested_vec_owned!["SIX", "SEVEN", "SEVEN"];
        let result = "TWENTY".to_string();
        assert!(Solution::is_solvable(words, result));
    }

    #[test]
    fn sample3() {
        let words = nested_vec_owned!["THIS", "IS", "TOO"];
        let result = "FUNNY".to_string();
        assert!(Solution::is_solvable(words, result));
    }

    #[test]
    fn sample4() {
        let words = nested_vec_owned!["LEET", "CODE"];
        let result = "POINT".to_string();
        assert!(!Solution::is_solvable(words, result));
    }

    #[test]
    fn issue1() {
        let words = nested_vec_owned!["AB", "CD", "EF"];
        let result = "GHIJ".to_string();
        assert!(!Solution::is_solvable(words, result));
    }

    #[test]
    fn issue2() {
        let words = nested_vec_owned!["A", "B"];
        let result = "A".to_string();
        assert!(Solution::is_solvable(words, result));
    }

    #[test]
    fn test() {
        let a = Some(Some(1i32));
        if a.is_some()
            && let Some(b) = a
            && let Some(c) = b
        {
            assert_eq!(c, 1);
        }
    }
}
