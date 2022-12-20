use std::fmt::Debug;

pub fn split_every<T>(xs: Vec<T>, n: usize) -> Vec<Vec<T>>
where
    T: Clone + Debug,
{
    let iter = xs.clone().into_iter();
    let mut result: Vec<Vec<T>> = vec![Vec::with_capacity(n); xs.len() / n];

    for (i, x) in iter.enumerate() {
        result.get_mut(i / n).map(|v| v.insert(i % n, x));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_every() {
        let result = split_every(vec![0; 8], 4);

        assert_eq!(result, vec![vec![0; 4], vec![0; 4]]);

        let result = split_every(vec!["x"; 8], 2);

        assert_eq!(
            result,
            vec![vec!["x"; 2], vec!["x"; 2], vec!["x"; 2], vec!["x"; 2],]
        );
    }
}
