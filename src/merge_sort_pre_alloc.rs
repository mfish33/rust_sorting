use super::{Sorter, Sortable};

pub struct MergeSortPreAlloc;

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


impl MergeSortPreAlloc {

    pub fn sort_runner<T:Sortable>(&self, slice: &mut [T], left_buffer:&mut Vec<T>, right_buffer:&mut Vec<T>) {
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
        self.sort_runner(left, left_buffer, right_buffer);
        self.sort_runner(right, left_buffer, right_buffer);
        self.merge(slice, mid, left_buffer, right_buffer)
    }


    pub fn merge<T:Sortable>(&self, slice: &mut [T], mid:usize, left_buffer:&mut Vec<T>, right_buffer:&mut Vec<T>) {
        let (left, right) = slice.split_at_mut(mid);
        for item in left.iter().rev() {
            left_buffer.push(item.clone());
        }
        for item in right.iter().rev() {
            right_buffer.push(item.clone());
        }
        for i in 0..slice.len() {
            if right_buffer.last().is_none() || left_buffer.last().is_some() && left_buffer.last().unwrap() < right_buffer.last().unwrap() {
                slice[i] = left_buffer.pop().unwrap();
            } else {
                slice[i] = right_buffer.pop().unwrap();
            }
        }
    
    }
}

impl Sorter for MergeSortPreAlloc {
    fn sort<T>(&self, slice: &mut [T]) where T:Sortable {
        let buffer_size = slice.len() / 2 + 1;
        let ref mut left_buffer = Vec::with_capacity(buffer_size);
        let ref mut right_buffer = Vec::with_capacity(buffer_size);
        self.sort_runner(slice, left_buffer, right_buffer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut arr = vec![0,4,5,1,2,7];
        MergeSortPreAlloc.sort(&mut arr);
        assert_eq!(arr, &[0,1,2,4,5,7])
    }
}