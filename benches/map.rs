use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lock_freedom::map::Map;

fn random_read_write(n: u64) {
    let mut map = Map::new();
    for i in 0..n {
        map.insert(i, i);
    }
    for i in 0..n {
        assert_eq!(map.get(&i).unwrap().as_ref().1, i);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("read write 20", |b| {
        b.iter(|| random_read_write(black_box(20)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
