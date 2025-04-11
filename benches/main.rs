use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use rstu::{
    BubbleSort, HeapSort, InsertionSort, MergeSort, QuickSort, SelectionSort, StableSorter,
    UnstableSorter,
};

fn get_random_slice(size: usize) -> Vec<u32> {
    (0..size).map(|_| rand::random()).collect()
}

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorts_comparison");
    group.sample_size(10);

    let sizes = [10, 100, 1000, 10_000, 100_000];

    for &size in &sizes {
        let slice = get_random_slice(size);

        group.bench_with_input(BenchmarkId::new("quick", size), &slice, |b, data| {
            b.iter(|| QuickSort::sort_unstable(black_box(&mut data.clone())))
        });
        group.bench_with_input(BenchmarkId::new("heap", size), &slice, |b, data| {
            b.iter(|| HeapSort::sort_unstable(black_box(&mut data.clone())))
        });
        group.bench_with_input(BenchmarkId::new("merge", size), &slice, |b, data| {
            b.iter(|| MergeSort::sort(black_box(&mut data.clone())))
        });
        group.bench_with_input(BenchmarkId::new("insertion", size), &slice, |b, data| {
            b.iter(|| InsertionSort::sort(black_box(&mut data.clone())))
        });
        group.bench_with_input(BenchmarkId::new("selection", size), &slice, |b, data| {
            b.iter(|| SelectionSort::sort_unstable(black_box(&mut data.clone())))
        });
        group.bench_with_input(BenchmarkId::new("bubble", size), &slice, |b, data| {
            b.iter(|| BubbleSort::sort(black_box(&mut data.clone())))
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
