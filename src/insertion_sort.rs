use super::{Sorter, Sortable};

pub struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort<T>(&self, slice: &mut [T]) where T:Sortable {
        for i in 1..slice.len() {
            let mut swap_index = i;
            while swap_index > 0 && slice[swap_index] < slice[swap_index-1] {
                slice.swap(swap_index, swap_index - 1);
                swap_index -= 1;
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
        InsertionSort.sort(&mut arr);
        assert_eq!(arr, &[1,2,4,5])
    }
}