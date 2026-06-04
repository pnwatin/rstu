use crate::bench_sorter;
use rstu::{HeapSort, UnstableSorter};

bench_sorter!(HeapSort, sort_unstable, "heapsort", LARGE_SIZES);
