pub struct Solution;

impl Solution {
    pub fn closest_room(mut rooms: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        use std::cmp::Reverse;
        rooms.sort_unstable_by_key(|r| Reverse(r[1]));

        let sorted_query_indices = {
            let mut indices = (0..queries.len()).collect::<Vec<_>>();
            indices.sort_unstable_by_key(|idx| Reverse(queries[*idx][1]));
            indices
        };

        use std::collections::BTreeSet;
        let mut set = BTreeSet::new();
        let mut answer = vec![-1; queries.len()];
        let mut room_id = 0;
        for qid in sorted_query_indices {
            let query = &queries[qid];
            while room_id < rooms.len() && rooms[room_id][1] >= query[1] {
                set.insert(rooms[room_id][0]);
                room_id += 1;
            }
            let mut id = i32::MIN;
            if let Some(v) = set.range(..query[0]).next_back()
                && (id == i32::MIN || (query[0] - *v).abs() < (query[0] - id).abs())
            {
                id = *v;
            }
            if let Some(v) = set.range(query[0]..).next()
                && (id == i32::MIN || (query[0] - *v).abs() < (query[0] - id).abs())
            {
                id = *v;
            }
            answer[qid] = if id == i32::MIN { -1 } else { id };
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let rooms = nested_vec![[2, 2], [1, 2], [3, 2]];
        let queries = nested_vec![[3, 1], [3, 3], [5, 2]];
        assert_eq!(Solution::closest_room(rooms, queries), vec![3, -1, 3])
    }

    #[test]
    fn sample2() {
        let rooms = nested_vec![[1, 4], [2, 3], [3, 5], [4, 1], [5, 2]];
        let queries = nested_vec![[2, 3], [2, 4], [2, 5]];
        assert_eq!(Solution::closest_room(rooms, queries), vec![2, 1, 3])
    }
}
