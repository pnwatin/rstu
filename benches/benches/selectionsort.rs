use crate::bench_sorter;
use rstu::{SelectionSort, UnstableSorter};

bench_sorter!(SelectionSort, sort_unstable, "selectionsort", SMALL_SIZES);
