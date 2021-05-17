use super::{Sorter, Sortable};

pub struct QuickSort;

impl Sorter for QuickSort {
    fn sort<T>(&self, slice: &mut [T]) where T:Sortable {
        match slice.len() {
            0 | 1 => return,
            2 => {
                if slice[0] > slice[1] {
                    slice.swap(0, 1)
                }
                return
            },
            _ => ()
        }
        let (pivot, rest) = slice.split_at_mut(1);
        let pivot = &pivot[0];
        let mut left = 0;
        let mut right = rest.len() - 1;
        while left <= right {
            if &rest[left] < pivot {
                left += 1;
            } else if &rest[right] > pivot {
                if right == 0 {
                    break;
                }
                right -= 1;
            } else {
                rest.swap(left, right);
                left += 1;
                if right == 0 {
                    break;
                }
                right -= 1;
            }
        }
        slice.swap(0, left);
        self.sort(&mut slice[..left]);
        self.sort(&mut slice[left+1..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut arr = vec![0,4,5,1,2,7];
        QuickSort.sort(&mut arr);
        assert_eq!(arr, &[0,1,2,4,5,7])
    }
}