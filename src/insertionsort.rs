use crate::StableSorter;

pub struct InsertionSort;

impl<T> StableSorter<T> for InsertionSort
where
    T: std::cmp::Ord,
{
    fn sort(slice: &mut [T]) {
        if slice.len() < 2 {
            return;
        }

        for unsorted in 1..slice.len() {
            let mut i = unsorted;

            while i > 0 && slice[i] < slice[i - 1] {
                slice.swap(i, i - 1);
                i -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{StableChecker, sorter_stable_check};
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn insertion_works(slice: Vec<u32>) -> bool {
        let mut slice = slice;
        InsertionSort::sort(&mut slice);

        slice.is_sorted()
    }

    #[quickcheck]
    fn insertion_is_stable(slice: Vec<StableChecker>) -> bool {
        sorter_stable_check::<InsertionSort>(slice)
    }
}
