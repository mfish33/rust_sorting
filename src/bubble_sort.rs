use super::{Sorter, Sortable};

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(&self, slice: &mut [T]) where T:Sortable {
        let mut some_swap = true;
        while some_swap {
            some_swap = false;
            for i in 1..slice.len() {
                if slice[i] < slice[i -1] {
                    some_swap = true;
                    slice.swap(i, i - 1)
                } 
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
        BubbleSort.sort(&mut arr);
        assert_eq!(arr, &[1,2,4,5])
    }
}
