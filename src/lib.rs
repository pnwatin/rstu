mod bubblesort;
mod heapsort;
mod insertionsort;
mod quicksort;
mod selectionsort;

pub use bubblesort::BubbleSort;
pub use heapsort::HeapSort;
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
pub mod tests {
    use super::*;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn stable_std_works(slice: Vec<u32>) -> bool {
        let mut slice = slice;
        StdSorter::sort(&mut slice);

        slice.is_sorted()
    }

    #[quickcheck]
    fn unstable_std_works(slice: Vec<u32>) -> bool {
        let mut slice = slice;
        StdSorter::sort_unstable(&mut slice);

        slice.is_sorted()
    }

    #[quickcheck]
    fn stable_std_is_stable(slice: Vec<StableChecker>) -> bool {
        sorter_stable_check::<StdSorter>(slice)
    }

    pub fn sorter_stable_check<S>(slice: Vec<StableChecker>) -> bool
    where
        S: StableSorter<StableChecker>,
    {
        if slice.is_empty() {
            return true;
        }

        let mut slice = slice;

        for (i, sc) in slice.iter_mut().enumerate() {
            sc.id = i as u32;
        }

        S::sort(&mut slice);

        for i in 0..slice.len() - 1 {
            let curr = &slice[i];
            let next = &slice[i + 1];

            if curr.key == next.key && curr.id > next.id {
                return false;
            }
        }

        slice.is_sorted()
    }

    #[derive(Clone, Debug)]
    pub struct StableChecker {
        id: u32,
        key: u8,
    }

    impl std::cmp::PartialEq for StableChecker {
        fn eq(&self, other: &Self) -> bool {
            self.key == other.key
        }
    }

    impl std::cmp::Eq for StableChecker {}

    impl std::cmp::PartialOrd for StableChecker {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl std::cmp::Ord for StableChecker {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.key.cmp(&other.key)
        }
    }

    impl Arbitrary for StableChecker {
        fn arbitrary(g: &mut Gen) -> Self {
            StableChecker {
                id: u32::arbitrary(g),
                key: u8::arbitrary(g) % 10,
            }
        }
    }
}
