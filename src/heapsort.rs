use crate::UnstableSorter;

pub struct HeapSort;

fn heapify<T>(slice: &mut [T])
where
    T: std::cmp::Ord,
{
    for i in (0..slice.len() / 2).rev() {
        sift_down(slice, i);
    }
}

fn sift_down<T>(slice: &mut [T], root: usize)
where
    T: std::cmp::Ord,
{
    let mut largest = root;
    let left = 2 * root + 1;
    let right = left + 1;
    let n = slice.len();

    if left < n && slice[left] > slice[largest] {
        largest = left;
    }

    if right < n && slice[right] > slice[largest] {
        largest = right;
    }

    if largest != root {
        slice.swap(largest, root);
        sift_down(slice, largest);
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

        for unsorted in (0..slice.len()).rev() {
            slice.swap(0, unsorted);
            sift_down(&mut slice[..unsorted], 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn heap_works(slice: Vec<u32>) -> bool {
        let mut slice = slice;
        HeapSort::sort_unstable(&mut slice);

        slice.is_sorted()
    }
}
