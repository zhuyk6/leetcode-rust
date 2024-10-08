/// Create a nested [Vec].
#[macro_export]
macro_rules! nested_vec {
    [$( $([ $($elems:tt)* ]),+ $(,)? )?] => {
        vec![$( $(nested_vec![$($elems)*]),+)?]
    };
    [$($elems:tt)*] => {
        vec![$($elems)*]
    };
}

#[allow(unused_imports)]
pub(crate) use nested_vec;

/// Create a nested [Vec] which contains owned value.
#[macro_export]
macro_rules! nested_vec_owned {
    [$( $([ $($elems:tt)* ]),+ $(,)? )?] => {
        vec![$( $(nested_vec_owned![$($elems)*]),+)?]
    };
    [$($elems:expr),* $(,)?] => {
        vec![$($elems.to_owned()),*]
    };
}

#[allow(unused_imports)]
pub(crate) use nested_vec_owned;

#[cfg(test)]
mod test_nested_vec {
    use super::nested_vec;

    #[test]
    fn test_empty() {
        let empty: Vec<i32> = nested_vec![];
        assert_eq!(empty, Vec::new());
    }

    #[test]
    fn test_1d() {
        assert_eq!(nested_vec![1, 2, 3], vec![1, 2, 3]);
    }

    #[test]
    fn test_2d() {
        assert_eq!(
            nested_vec![[1, 2, 3], [4, 5]],
            vec![vec![1, 2, 3], vec![4, 5]]
        );
    }

    #[test]
    fn test_3d() {
        let left = nested_vec![[[1, 2]], [[4], [5], []], [[], [6, 7]]];
        assert_eq!(
            left,
            vec![
                vec![vec![1, 2]],
                vec![vec![4], vec![5], vec![]],
                vec![vec![], vec![6, 7]]
            ]
        );
    }
}

#[cfg(test)]
mod test_nested_vec_owned {
    use super::*;

    #[test]
    fn test_empty() {
        let left: Vec<i32> = nested_vec_owned![];
        let right: Vec<i32> = vec![];
        assert_eq!(left, right);
    }

    #[test]
    fn test_1d() {
        let left: Vec<String> = nested_vec_owned!["a", "b", "c"];
        let right: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(left, right);
    }

    #[test]
    fn test_2d() {
        let left: Vec<Vec<String>> =
            nested_vec_owned![["a".to_string(), "b".to_string()], ["c".to_string()]];
        let right: Vec<Vec<String>> = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string()],
        ];
        assert_eq!(left, right);
    }
}
