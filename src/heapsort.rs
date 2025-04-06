use crate::UnstableSorter;

pub struct HeapSort;

fn heapify<T>(slice: &mut [T])
where
    T: std::cmp::Ord,
{
    let end = slice.len();

    for i in (0..(end - 1) / 2 + 1).rev() {
        sift_down(slice, i, end);
    }
}

fn sift_down<T>(slice: &mut [T], root: usize, end: usize)
where
    T: std::cmp::Ord,
{
    let mut root = root;

    loop {
        let left = 2 * root + 1;
        let right = left + 1;

        if left >= end {
            return;
        }

        let child = if right < end && slice[left] < slice[right] {
            right
        } else {
            left
        };

        if slice[root] >= slice[child] {
            return;
        }

        slice.swap(root, child);

        root = child;
    }
}

impl<T> UnstableSorter<T> for HeapSort
where
    T: std::cmp::Ord,
{
    fn sort_unstable(slice: &mut [T]) {
        if slice.len() < 2 {
            return;
        }

        heapify(slice);

        for end in (1..slice.len()).rev() {
            slice.swap(end, 0);
            sift_down(slice, 0, end);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::quickcheck;

    quickcheck! {
        fn heap_works(slice: Vec<u32>) -> bool {
            let mut slice =  slice;
            HeapSort::sort_unstable(&mut slice);

            slice.is_sorted()
        }
    }
}
