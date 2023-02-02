use criterion::{criterion_group, criterion_main, black_box, Criterion};

use gdlauncher_entry::*;

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("All");

    group.bench_function("no_idx",           |b| b.iter(|| black_box(no_idx::solve())));
    group.bench_function("no_dups",          |b| b.iter(|| black_box(no_dups::solve())));
    group.bench_function("cyclic",           |b| b.iter(|| black_box(cyclic::solve())));
    group.bench_function("w_counter",        |b| b.iter(|| black_box(w_counter::solve())));
    group.bench_function("counter_n_cyclic", |b| b.iter(|| black_box(counter_n_cyclic::solve())));

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
