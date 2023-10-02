pub mod insertion_sort;

#[cfg(test)]
fn is_sorted(array: &[i32]) -> bool {
    for i in 1..array.len() {
        if array[i - 1] > array[i] {
            return false;
        }
    }

    true
}
