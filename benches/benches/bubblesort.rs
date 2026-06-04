use crate::bench_sorter;
use rstu::{BubbleSort, StableSorter};

bench_sorter!(BubbleSort, sort, "bubblesort", SMALL_SIZES);
