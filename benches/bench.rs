mod benches;
pub mod common;

use benches::{bubblesort, heapsort, insertionsort, mergesort, quicksort, selectionsort};
use criterion::criterion_main;

criterion_main!(
    quicksort::benches,
    heapsort::benches,
    mergesort::benches,
    insertionsort::benches,
    selectionsort::benches,
    bubblesort::benches
);
