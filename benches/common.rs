use rand::{Rng, SeedableRng, rngs::StdRng};

pub const LARGE_SIZES: &[usize] = &[10, 100, 1_000, 10_000, 100_000, 1_000_000];
pub const SMALL_SIZES: &[usize] = &[10, 50, 100, 500, 1000];

pub const ADVERSIAL_PATTERNS: [AdversialPattern; 4] = [
    ("sorted", sorted),
    ("reverse", reverse),
    ("duplicates", duplicates),
    ("partial_sorted", partial_sorted),
];
pub fn rand_entropy(size: usize) -> Vec<u32> {
    (0..size).map(|_| rand::random()).collect()
}

pub fn rand_fixed(size: usize, seed: u64) -> Vec<u32> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..size).map(|_| rng.next_u32()).collect()
}

#[macro_export]
macro_rules! bench_sorter {
    ($ty:ty, $method:ident, $slug:expr, $sizes:ident) => {
        use criterion::{BatchSize, BenchmarkId, Criterion, Throughput};

        fn bench_random_entropy(c: &mut Criterion) {
            let mut g = c.benchmark_group(concat!($slug, "-random-entropy"));
            for &n in $crate::common::$sizes {
                g.throughput(Throughput::Elements(n as u64));
                g.bench_function(BenchmarkId::from_parameter(n), |b| {
                    b.iter_batched(
                        || $crate::common::rand_entropy(n),
                        |mut data| <$ty>::$method(std::hint::black_box(&mut data)),
                        BatchSize::SmallInput,
                    )
                });
            }
            g.finish();
        }

        fn bench_random_fixed(c: &mut Criterion) {
            let mut g = c.benchmark_group(concat!($slug, "-random-fixed"));
            for &n in $crate::common::$sizes {
                let template = $crate::common::rand_fixed(n, 0);
                g.throughput(Throughput::Elements(n as u64));
                g.bench_function(BenchmarkId::from_parameter(n), |b| {
                    b.iter_batched(
                        || template.clone(),
                        |mut data| <$ty>::$method(std::hint::black_box(&mut data)),
                        BatchSize::SmallInput,
                    )
                });
            }
            g.finish();
        }

        fn bench_adversarial(c: &mut Criterion) {
            let mut g = c.benchmark_group(concat!($slug, "-adversarial"));
            for (label, gen_vec) in $crate::common::ADVERSIAL_PATTERNS {
                for &n in $crate::common::$sizes {
                    let template = gen_vec(n);
                    g.throughput(Throughput::Elements(n as u64));
                    g.bench_function(BenchmarkId::new(label, n), |b| {
                        b.iter_batched(
                            || template.clone(),
                            |mut data| <$ty>::$method(std::hint::black_box(&mut data)),
                            BatchSize::SmallInput,
                        )
                    });
                }
            }
            g.finish();
        }

        criterion::criterion_group!(
            benches,
            bench_random_entropy,
            bench_random_fixed,
            bench_adversarial
        );
        criterion::criterion_main!(benches);
    };
}

type AdversialPattern = (&'static str, fn(usize) -> Vec<u32>);

fn sorted(size: usize) -> Vec<u32> {
    (0..size as u32).collect()
}

fn reverse(size: usize) -> Vec<u32> {
    (0..size as u32).rev().collect()
}

fn duplicates(size: usize) -> Vec<u32> {
    (0..size).map(|_| rand::random_range(0..1024)).collect()
}

fn partial_sorted(size: usize) -> Vec<u32> {
    let mut v: Vec<u32> = (0..size as u32).collect();
    for _ in 0..size / 10 {
        v.swap(rand::random_range(0..size), rand::random_range(0..size));
    }
    v
}
