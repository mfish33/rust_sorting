use super::{Sorter, Sortable};

pub struct StdSort;
impl Sorter for StdSort {
    fn sort<T>(&self, slice: &mut [T]) where T:Sortable {
        slice.sort()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut arr = vec![4,5,1,2];
        StdSort.sort(&mut arr);
        assert_eq!(arr, &[1,2,4,5])
    }
}