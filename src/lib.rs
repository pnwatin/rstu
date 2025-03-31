mod bubblesort;
mod insertionsort;

pub use bubblesort::BubbleSort;
pub use insertionsort::InsertionSort;

pub trait StableSorter {
    fn sort<T>(slice: &mut [T])
    where
        T: std::cmp::Ord;
}

pub trait UnstableSorter {
    fn sort_unstable<T>(slice: &mut [T])
    where
        T: std::cmp::Ord;
}

pub struct StdSorter;

impl StableSorter for StdSorter {
    fn sort<T>(slice: &mut [T])
    where
        T: std::cmp::Ord,
    {
        slice.sort();
    }
}

impl UnstableSorter for StdSorter {
    fn sort_unstable<T>(slice: &mut [T])
    where
        T: std::cmp::Ord,
    {
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
