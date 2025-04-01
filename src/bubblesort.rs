use crate::StableSorter;

pub struct BubbleSort;

impl<T> StableSorter<T> for BubbleSort
where
    T: std::cmp::Ord,
{
    fn sort(slice: &mut [T]) {
        if slice.len() < 2 {
            return;
        }

        let mut n = slice.len();

        while n > 1 {
            let mut new_n = 0;

            for i in 1..n {
                if slice[i] < slice[i - 1] {
                    slice.swap(i, i - 1);
                    new_n = i;
                }
            }

            n = new_n;
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
