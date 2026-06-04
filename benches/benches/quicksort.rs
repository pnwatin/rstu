use crate::bench_sorter;
use rstu::{QuickSort, UnstableSorter};

bench_sorter!(QuickSort, sort_unstable, "quicksort", LARGE_SIZES);
