use std::collections::VecDeque;

#[allow(dead_code)]
pub fn merge_sorted<T>(mut left: VecDeque<T>, mut right: VecDeque<T>) -> VecDeque<T>
where T: Ord {
    let mut res: VecDeque<T> = VecDeque::new();
    while !left.is_empty() && !right.is_empty() {
        if left.front() > right.front() {
            res.push_back(right.pop_front().unwrap())
        } else {
            res.push_back( left.pop_front().unwrap())
        }
    };
    res.append(&mut left);
    res.append(&mut right);
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn merge_sorted_equal_arrays() {
        let left = VecDeque::from_iter(vec![1, 12, 23]);
        let right = VecDeque::from_iter(vec![10, 22, 40]);
        assert_eq!(merge_sorted(left, right), VecDeque::from_iter(vec![1, 10, 12, 22, 23, 40]));
    }

    #[test]
    fn merge_sorted_non_equal_arrays() {
        let left = VecDeque::from_iter(vec![1]);
        let right = VecDeque::from_iter(vec![10, 22, 40]);
        assert_eq!(merge_sorted(left, right), VecDeque::from_iter(vec![1, 10, 22, 40]));
    }

    #[test]
    fn merge_sorted_empty_array() {
        let left = VecDeque::new();
        let right = VecDeque::from_iter(vec![10, 22, 40]);
        assert_eq!(merge_sorted(left, right), VecDeque::from_iter(vec![10, 22, 40]));
    }

    #[test]
    fn merge_sorted_empty_arrays() {
        let left: VecDeque<i32> = VecDeque::new();
        let right: VecDeque<i32> = VecDeque::new();
        assert_eq!(merge_sorted(left, right), VecDeque::<i32>::new());
    }
}