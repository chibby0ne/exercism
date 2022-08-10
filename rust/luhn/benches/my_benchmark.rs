use criterion::{black_box, criterion_group, criterion_main, Criterion};
use luhn::{is_valid, is_valid2, is_valid3};



fn bench_is_valid(c: &mut Criterion) {
    c.bench_function("is_valid", |b| b.iter(|| is_valid(black_box("234 567 891 234"))));
    c.bench_function("is_valid2", |b| b.iter(|| is_valid2(black_box("234 567 891 234"))));
    c.bench_function("is_valid3", |b| b.iter(|| is_valid2(black_box("234 567 891 234"))));
}

criterion_group!(benches, bench_is_valid);
criterion_main!(benches);

