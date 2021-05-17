use super::{Sorter, MergeSort, Sortable, MergeSortPreAlloc};
use rayon::prelude::*;

pub struct ParMergeSort;


impl Sorter for ParMergeSort {
    fn sort<T>(&self, slice: &mut [T]) where T:Sortable {

        if slice.len() == 0 {
            return
        }

        let mut threads = rayon::current_num_threads() - 1;
        if slice.len() / threads <= 2 {
            threads = 1;
        }

        let chunk_size = slice.len() / threads + 1;
        slice
        .par_chunks_mut(chunk_size)
        .for_each(|slice|{
            MergeSortPreAlloc.sort(slice);
        });

        for i in 2..=threads {
            let arr_end = if chunk_size * i < slice.len() {
                chunk_size * i
            } else {
                slice.len()
            };
            MergeSort.merge(&mut slice[0..arr_end], chunk_size * (i-1))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut arr = vec![0,4,5,1,2,7,10,8,9,10,6,1,-10,-2,-3,-4,-16];
        ParMergeSort.sort(&mut arr);
        assert_eq!(arr, &[-16,-10,-4,-3,-2,0,1,1,2,4,5,6,7,8,9,10,10])
    }
}