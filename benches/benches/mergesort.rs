use crate::bench_sorter;
use rstu::{MergeSort, StableSorter};

bench_sorter!(MergeSort, sort, "mergesort", LARGE_SIZES);
