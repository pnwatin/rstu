use crate::StableSorter;

pub struct MergeSort;

fn mergesort<T>(slice: &mut [T])
where
    T: std::cmp::Ord + Copy,
{
    if slice.len() < 2 {
        return;
    }

    let (left, right) = slice.split_at_mut(slice.len() / 2);

    mergesort(left);
    mergesort(right);

    let mut temp_vec = Vec::with_capacity(left.len() + right.len());
    merge_slices_in_places(left, right, &mut temp_vec);

    slice.copy_from_slice(&temp_vec);
}

fn merge_slices_in_places<T>(left: &mut [T], right: &mut [T], temp_vec: &mut Vec<T>)
where
    T: std::cmp::Ord + Copy,
{
    let mut l = 0;
    let mut r = 0;

    while l < left.len() && r < right.len() {
        if left[l] <= right[r] {
            temp_vec.push(left[l]);
            l += 1;
        } else {
            temp_vec.push(right[r]);
            r += 1;
        }
    }

    temp_vec.extend(&left[l..]);
    temp_vec.extend(&right[r..]);
}

impl<T> StableSorter<T> for MergeSort
where
    T: std::cmp::Ord + Copy,
{
    fn sort(slice: &mut [T]) {
        mergesort(slice);
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::{StableChecker, sorter_stable_check};

    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn merge_works(slice: Vec<u32>) -> bool {
        let mut slice = slice;
        MergeSort::sort(&mut slice);

        slice.is_sorted()
    }

    #[quickcheck]
    fn merge_is_stable(slice: Vec<StableChecker>) -> bool {
        sorter_stable_check::<MergeSort>(slice)
    }
}
