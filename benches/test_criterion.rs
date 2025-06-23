use criterion::{criterion_group, criterion_main, Criterion, BatchSize};
use std::hint::black_box;

fn setup_100_000_000(c: &mut Criterion) {
    const SETUP: i32 = 100_000_000;
    let setup = || {
        let mut i = 0;
        let mut x = 1;
        while i < SETUP {
            x += black_box(i * x);
            i += 1;
        }
        black_box(i)
    };

    let f = |_| {
        let mut j = 0;
        let mut x = 1;
        while j < 2 {
            x += black_box(x + j);
            j += black_box(1);
        }
    };

    c.bench_function("setup_100_000_000", move |b| b.iter_batched(setup, f, BatchSize::SmallInput));
}

fn setup_1_000_000(c: &mut Criterion) {
    const SETUP: i32 = 1_000_000;
    let setup = || {
        let mut i = 0;
        let mut x = 1;
        while i < SETUP {
            x += black_box(i * x);
            i += 1;
        }
        black_box(i)
    };

    let f = |_| {
        let mut j = 0;
        let mut x = 1;
        while j < 2 {
            x += black_box(x + j);
            j += black_box(1);
        }
    };

    c.bench_function("setup_1_000_000", move |b| b.iter_batched(setup, f, BatchSize::SmallInput));
}

fn setup_1_000(c: &mut Criterion) {
    const SETUP: i32 = 1_000;
    let setup = || {
        let mut i = 0;
        let mut x = 1;
        while i < SETUP {
            x += black_box(i * x);
            i += 1;
        }
        black_box(i)
    };

    let f = |_| {
        let mut j = 0;
        let mut x = 1;
        while j < 2 {
            x += black_box(x + j);
            j += black_box(1);
        }
    };

    c.bench_function("setup_1_000", move |b| b.iter_batched(setup, f, BatchSize::SmallInput));
}

fn setup_10(c: &mut Criterion) {
    const SETUP: i32 = 10;
    let setup = || {
        let mut i = 0;
        let mut x = 1;
        while i < SETUP {
            x += black_box(i * x);
            i += 1;
        }
        black_box(i)
    };

    let f = |_| {
        let mut j = 0;
        let mut x = 1;
        while j < 2 {
            x += black_box(x + j);
            j += black_box(1);
        }
    };

    c.bench_function("setup_10", move |b| b.iter_batched(setup, f, BatchSize::SmallInput));
}

fn setup_1(c: &mut Criterion) {
    const SETUP: i32 = 1;
    let setup = || {
        let mut i = 0;
        let mut x = 1;
        while i < SETUP {
            x += black_box(i * x);
            i += 1;
        }
        black_box(i)
    };

    let f = |_| {
        let mut j = 0;
        let mut x = 1;
        while j < 2 {
            x += black_box(x + j);
            j += black_box(1);
        }
    };

    c.bench_function("setup_1", move |b| b.iter_batched(setup, f, BatchSize::SmallInput));
}


criterion_group!(
    benches,
    setup_1,
    setup_10,
    setup_1_000,
    setup_1_000_000,
    setup_100_000_000,
);
criterion_main!(benches);
