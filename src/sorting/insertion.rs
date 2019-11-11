#[allow(dead_code)]
pub fn insertion_sort<T>(mut values: Vec<T>) -> Vec<T>
where T: Ord {
    for i in 1..values.len() {
        for j in (0..i).rev() {
            if values[j] > values[j + 1] {
                values.swap(j, j + 1);
            } else {
                break
            };
        }
    };
    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let arr: Vec<i32> = Vec::new();
        assert_eq!(insertion_sort(arr), vec![]);
    }

    #[test]
    fn numbers() {
        let arr = vec![5, 12, -1, 15, 10, 1, 3, 0];
        assert_eq!(insertion_sort(arr), vec![-1, 0, 1, 3, 5, 10, 12, 15]);
    }

    #[test]
    fn chars() {
        let arr = vec!["b", "a", "c"];
        assert_eq!(insertion_sort(arr), vec!["a", "b", "c"]);
    }

}
