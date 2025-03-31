mod bubblesort;

pub use bubblesort::BubbleSort;

pub trait Sorter {
    fn sort<T>(slice: &mut [T])
    where
        T: std::cmp::Ord;
}

pub struct StdSorter;

impl Sorter for StdSorter {
    fn sort<T>(slice: &mut [T])
    where
        T: std::cmp::Ord,
    {
        slice.sort();
    }
}

pub struct StdUnstableSorter;

impl Sorter for StdUnstableSorter {
    fn sort<T>(slice: &mut [T])
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
        fn std_works(slice: Vec<u32>) -> bool {
            let mut slice =  slice;
            StdSorter::sort(&mut slice);

            slice.is_sorted()
        }
    }

    quickcheck! {
        fn std_unstable_works(slice: Vec<u32>) -> bool {
            let mut slice =  slice;
            StdUnstableSorter::sort(&mut slice);

            slice.is_sorted()
        }
    }
}
