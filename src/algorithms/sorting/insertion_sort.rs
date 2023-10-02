pub fn insertion_sort<T: Ord + Copy>(values: &mut [T]) {
    for i in 1..values.len() {
        let key = values[i];
        let mut j = i - 1;

        // Move elements of arr[0..i-1] that are greater than the key, to one position ahead of
        // their current position.
        while values[j] > key {
            values.swap(j + 1, j);
            j -= 1;
        }

        values[j + 1] = key;
    }
}

#[cfg(test)]
mod tests {
    use super::{super::is_sorted, insertion_sort};
    use rand::Rng;

    #[test]
    fn insertion() {
        let mut numbers = Vec::with_capacity(1_000_000);
        let mut rng = rand::thread_rng();

        for num in &mut numbers {
            *num = rng.gen_range(0..=1_000_000);
        }

        insertion_sort(&mut numbers);

        assert!(is_sorted(&numbers));
    }
}
