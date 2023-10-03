pub fn insertion<T: Ord + Copy>(values: &mut [T]) {
    for i in 1..values.len() {
        let key = values[i];
        let mut j = i;

        // Move elements of arr[0..i-1] that are greater than the key, to one position ahead of
        // their current position.
        while j > 0 && values[j - 1] > key {
            values.swap(j, j - 1);
            j -= 1;
        }

        values[j] = key;
    }
}

#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use rand::Rng;

    #[test]
    fn insertion() {
        let mut numbers = Vec::with_capacity(1_000_000);
        let mut rng = rand::thread_rng();

        for _ in 0..100_000 {
            numbers.push(rng.gen_range(0..=1_000_000));
        }

        super::insertion(&mut numbers);

        assert!(is_sorted(&numbers));
    }
}
