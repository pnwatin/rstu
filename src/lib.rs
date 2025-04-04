mod bubblesort;
mod insertionsort;
mod quicksort;
mod selectionsort;

pub use bubblesort::BubbleSort;
pub use insertionsort::InsertionSort;
pub use quicksort::QuickSort;
pub use selectionsort::SelectionSort;

pub trait StableSorter<T>
where
    T: std::cmp::Ord,
{
    fn sort(slice: &mut [T]);
}

pub trait UnstableSorter<T>
where
    T: std::cmp::Ord,
{
    fn sort_unstable(slice: &mut [T]);
}

pub struct StdSorter;

impl<T> StableSorter<T> for StdSorter
where
    T: std::cmp::Ord,
{
    fn sort(slice: &mut [T]) {
        slice.sort();
    }
}

impl<T> UnstableSorter<T> for StdSorter
where
    T: std::cmp::Ord,
{
    fn sort_unstable(slice: &mut [T]) {
        slice.sort_unstable();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::quickcheck;

    quickcheck! {
        fn stable_std_works(slice: Vec<u32>) -> bool {
            let mut slice =  slice;
            StdSorter::sort(&mut slice);

            slice.is_sorted()
        }
    }

    quickcheck! {
        fn unstable_std_works(slice: Vec<u32>) -> bool {
            let mut slice =  slice;
            StdSorter::sort_unstable(&mut slice);

            slice.is_sorted()
        }
    }
}
