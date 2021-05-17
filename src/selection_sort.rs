use super::{Sorter, Sortable};

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(&self, slice: &mut [T]) where T:Sortable {
        for i in 0..slice.len() {
            let mut min_remaining = i;
            for j in min_remaining + 1 .. slice.len() {
                if slice[j] < slice[min_remaining] {
                    min_remaining = j;
                }
            }
            if min_remaining != i {
                slice.swap(i, min_remaining);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut arr = vec![4,5,1,2];
        SelectionSort.sort(&mut arr);
        assert_eq!(arr, &[1,2,4,5])
    }
}