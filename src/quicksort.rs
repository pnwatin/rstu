use crate::{InsertionSort, StableSorter, UnstableSorter};

pub struct QuickSort;

const MAX_LEN_INSERTION_SORT: usize = 20;

fn quicksort<T>(slice: &mut [T])
where
    T: std::cmp::Ord,
{
    if slice.len() < 2 {
        return;
    }

    if slice.len() <= MAX_LEN_INSERTION_SORT {
        InsertionSort::sort(slice);
        return;
    }

    let mid = slice.len() / 2;
    slice.swap(0, mid);

    let (pivot, rest) = slice.split_first_mut().expect("slice should be non empty");

    let mut left = 0;
    let mut right = rest.len() - 1;

    while left <= right {
        if &rest[left] < pivot {
            left += 1;

            continue;
        }

        if &rest[right] <= pivot {
            rest.swap(left, right);
            left += 1;
        }

        if right == 0 {
            break;
        }

        right -= 1;
    }

    slice.swap(0, left);

    let (left, right) = slice.split_at_mut(left);

    quicksort(left);
    quicksort(&mut right[1..]);
}

impl<T> UnstableSorter<T> for QuickSort
where
    T: std::cmp::Ord,
{
    fn sort_unstable(slice: &mut [T]) {
        quicksort(slice);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn quick_works(slice: Vec<u32>) -> bool {
        let mut slice = slice;
        QuickSort::sort_unstable(&mut slice);

        slice.is_sorted()
    }
}
