use super::{Sorter, Sortable};

pub struct MergeSort;

trait ToVecRev {
    type Item;
    fn to_vec_rev(&self) -> Vec<Self::Item>;
}

impl<T:Clone> ToVecRev for &mut [T] {
    type Item = T;
    fn to_vec_rev(&self) -> Vec<T> {
        self.iter().rev().map(|v|v.clone()).collect()
    }
}


impl MergeSort {
    pub fn merge<T:Sortable>(&self, slice: &mut [T], mid:usize) {
        let (left, right) = slice.split_at_mut(mid);
        let mut left:Vec<T> = left.to_vec_rev();
        let mut right:Vec<T> = right.to_vec_rev();
        for i in 0..slice.len() {
            if right.last().is_none() || left.last().is_some() && left.last().unwrap() < right.last().unwrap() {
                slice[i] = left.pop().unwrap();
            } else {
                slice[i] = right.pop().unwrap();
            }
        }
    
    }
}

impl Sorter for MergeSort {
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
        let mid = slice.len() / 2;
        let (left, right) = slice.split_at_mut(mid);
        self.sort(left);
        self.sort(right);
        self.merge(slice, mid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut arr = vec![0,4,5,1,2,7];
        MergeSort.sort(&mut arr);
        assert_eq!(arr, &[0,1,2,4,5,7])
    }
}