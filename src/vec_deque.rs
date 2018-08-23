use std::collections::VecDeque;
use std::cmp::Ordering;
use std::mem;

pub trait VecDequeExt<T> {
    /// Sort the `VecDeque` using the insertion sort algorithm. The sorting algorithm can be
    /// quicker than the default algorithm on data which nearly sorted.
    fn insertion_sort_by<C>(&mut self, cmp: C)
    where
        C: FnMut(&T, &T) -> Ordering;
}

impl<T> VecDequeExt<T> for VecDeque<T> {
    fn insertion_sort_by<C>(&mut self, mut cmp: C)
    where
        C: FnMut(&T, &T) -> Ordering,
    {
        for i in 0..self.len() {
            let mut bottom = 0;
            for j in (0..i).rev() {
                match cmp(&self[j], &self[i]) {
                    Ordering::Greater => (),
                    _ => {
                        bottom = j + 1;
                        break;
                    },
                }
            }

            let (slice0, slice1) = self.as_mut_slices();
            let slice = match (bottom < slice0.len(), i < slice0.len()) {
                (true, true) => slice0,
                (false, false) => slice1,
                (true, false) => {
                    slice1[0..(i + 1)].rotate_right(1);
                    slice0[bottom..].rotate_right(1);
                    mem::swap(&mut slice0[0], &mut slice1[0]);
                    continue;
                },
                (false, true) => unreachable!(),
            };
            slice[bottom..(i + 1)].rotate_right(1);
        }
    }
}

