pub fn merge<T>(values: &mut [T])
where
    T: Ord + Copy,
{
    let mid = values.len() / 2;

    if values.len() <= 1 {
        return;
    }

    merge(&mut values[..mid]);
    merge(&mut values[mid..]);

    let (left, right) = (values[..mid].to_vec(), values[mid..].to_vec());
    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            values[k] = left[i];
            i += 1;
        } else {
            values[k] = right[j];
            j += 1;
        }

        k += 1;
    }

    while i < left.len() {
        values[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        values[k] = right[j];
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use rand::Rng;

    #[test]
    fn merge() {
        let mut numbers = Vec::with_capacity(1_000_000);
        let mut rng = rand::thread_rng();

        for _ in 0..100_000 {
            numbers.push(rng.gen_range(0..=1_000_000));
        }

        super::merge(&mut numbers);
        assert!(is_sorted(&numbers));
    }
}
