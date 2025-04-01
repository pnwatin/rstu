use crate::StableSorter;

pub struct InsertionSort;

impl<T> StableSorter<T> for InsertionSort
where
    T: std::cmp::Ord,
{
    fn sort(slice: &mut [T]) {
        if slice.len() < 2 {
            return;
        }

        for unsorted in 1..slice.len() {
            if slice[unsorted] >= slice[unsorted - 1] {
                continue;
            }

            for i in 0..unsorted {
                if slice[unsorted] < slice[i] {
                    slice[i..=unsorted].rotate_right(1);

                    break;
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
        fn insertion_works(slice: Vec<u32>) -> bool {
            let mut slice =  slice;
            InsertionSort::sort(&mut slice);

            slice.is_sorted()
        }
    }
}
