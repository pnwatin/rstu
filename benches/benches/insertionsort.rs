use crate::bench_sorter;
use rstu::{InsertionSort, StableSorter};

bench_sorter!(InsertionSort, sort, "insertionsort", SMALL_SIZES);
