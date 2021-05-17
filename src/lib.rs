mod bubble_sort;
mod insertion_sort;
mod selection_sort;
mod quick_sort;
mod merge_sort;
mod par_merge_sort;
mod merge_sort_pre_alloc;
mod std_sort;

pub use bubble_sort::BubbleSort;
pub use insertion_sort::InsertionSort;
pub use selection_sort::SelectionSort;
pub use quick_sort::QuickSort;
pub use merge_sort::MergeSort;
pub use par_merge_sort::ParMergeSort;
pub use merge_sort_pre_alloc::MergeSortPreAlloc;
pub use std_sort::StdSort;



// Trait Alias
use std::fmt::Debug;
pub trait Sortable: Ord + Clone + Send + Sync + Debug {}
impl<T> Sortable for T 
    where T: Ord + Clone + Send + Sync + Debug {}

pub trait Sorter {
    fn sort<T>(&self, slice: &mut [T]) where T:Sortable;
}



