use crate::StableSorter;

pub struct BubbleSort;

impl StableSorter for BubbleSort {
    fn sort<T>(slice: &mut [T])
    where
        T: std::cmp::Ord,
    {
        if slice.len() < 2 {
            return;
        }

        let mut swapped = true;

        while swapped {
            swapped = false;

            for i in 1..slice.len() {
                if slice[i] < slice[i - 1] {
                    slice.swap(i, i - 1);
                    swapped = true;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::quickcheck;

    quickcheck! {
        fn bubble_works(slice: Vec<u32>) -> bool {
            let mut slice =  slice;
            BubbleSort::sort(&mut slice);

            slice.is_sorted()
        }
    }
}
