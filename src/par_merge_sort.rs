use super::{Sorter, MergeSort, Sortable, MergeSortPreAlloc};
use std::sync::Arc;
use rayon::prelude::*;

pub struct ParMergeSort;


impl Sorter for ParMergeSort {
    fn sort<T>(&self, slice: &mut [T]) where T:Sortable {

        if slice.len() == 0 {
            return
        }

        let mut threads = rayon::current_num_threads();
        if slice.len() / threads <= 2 {
            threads = 1;
        }

        let chunk_size = slice.len() / threads + 1;
        slice
        .par_chunks_mut(chunk_size)
        .for_each(|chunk|{
            MergeSortPreAlloc.sort(chunk);
        });

        let mut chunks:Vec<_> = slice.chunks_mut(chunk_size).map(|chunk| Arc::new(chunk)).collect();

        while chunks.len() > 1 {
            chunks = chunks
            .par_chunks_mut(2)
            .map(|group|{
                if group.len() < 2 {
                    group[0].clone()
                } else {
                    let mid = group[0].len();
                    unsafe {
                        let slice = std::slice::from_raw_parts_mut(group[0].as_ptr() as *mut T, group[0].len() + group[1].len());
                        MergeSort.merge(slice, mid);
                        Arc::new(slice)
                    }
                    
                }
            }).collect();
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