use crate::UnstableSorter;

pub struct QuickSort;

fn quicksort<T>(slice: &mut [T])
where
    T: std::cmp::Ord,
{
    if slice.len() < 2 {
        return;
    }

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

impl UnstableSorter for QuickSort {
    fn sort_unstable<T>(slice: &mut [T])
    where
        T: std::cmp::Ord,
    {
        quicksort(slice);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::quickcheck;

    quickcheck! {
        fn quick_works(slice: Vec<u32>) -> bool {
            let mut slice =  slice;
            QuickSort::sort_unstable(&mut slice);

            slice.is_sorted()
        }
    }
}
