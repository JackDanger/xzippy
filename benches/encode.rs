use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

fn bench_encode(c: &mut Criterion) {
    // 64 KiB of zeros (highly compressible) as the input.
    let input = vec![0u8; 64 * 1024];

    let mut group = c.benchmark_group("encode");
    group.throughput(Throughput::Bytes(input.len() as u64));
    group.bench_function("lzma2_64k_zeros", |b| {
        b.iter(|| xzippy::encode::encode_7z(black_box(&input), 262144).unwrap());
    });
    group.finish();
}

criterion_group!(benches, bench_encode);
criterion_main!(benches);
