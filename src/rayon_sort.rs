use super::{Sortable, Sorter};
use rayon::prelude::ParallelSliceMut;

pub struct RayonSort;

impl Sorter for RayonSort {
    fn sort<T>(&self, slice: &mut [T]) where T:Sortable {
        slice.par_sort()
    }
}