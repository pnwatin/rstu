use crate::StableSorter;

pub struct SelectionSort;

impl<T> StableSorter<T> for SelectionSort
where
    T: std::cmp::Ord,
{
    fn sort(slice: &mut [T]) {
        if slice.len() < 2 {
            return;
        }

        for unsorted in 0..slice.len() - 1 {
            let mut smallest_in_rest = unsorted;

            for i in (unsorted + 1)..slice.len() {
                if slice[i] < slice[smallest_in_rest] {
                    smallest_in_rest = i;
                }
            }

            if smallest_in_rest != unsorted {
                slice.swap(unsorted, smallest_in_rest);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::quickcheck;

    quickcheck! {
        fn selection_works(slice: Vec<u32>) -> bool {
            let mut slice =  slice;
            SelectionSort::sort(&mut slice);

            slice.is_sorted()
        }
    }
}
