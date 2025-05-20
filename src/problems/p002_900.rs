pub struct Solution;

impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let n = words.len();
        let mut pre: Vec<Option<usize>> = vec![None; n];
        let mut last: [Option<usize>; 2] = [None, None];

        for (i, v) in groups.into_iter().enumerate() {
            let v = v as usize;
            pre[i] = last[v ^ 1];
            last[v] = Some(i);
        }

        let get_seq = |now: Option<usize>| -> Vec<usize> {
            let mut seq = vec![];
            let mut now = now;
            while let Some(i) = now {
                seq.push(i);
                now = pre[i];
            }
            seq.reverse();
            seq
        };

        let seq0 = get_seq(last[0]);
        let seq1 = get_seq(last[1]);

        let seq = if seq0.len() > seq1.len() { seq0 } else { seq1 };

        seq.into_iter().map(|i| words[i].clone()).collect()
    }
}
