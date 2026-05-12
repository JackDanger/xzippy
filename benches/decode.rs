use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

fn bench_decode(c: &mut Criterion) {
    // Encode 64 KiB of zeros (highly compressible) for a stable decode benchmark.
    let input = vec![0u8; 64 * 1024];
    let (props, compressed) = lazippier::encode::encode_7z(&input, 262144).unwrap();

    let mut group = c.benchmark_group("decode");
    group.throughput(Throughput::Bytes(input.len() as u64));
    group.bench_function("lzma2_64k_zeros", |b| {
        b.iter(|| {
            lazippier::decode::decode_7z(black_box(&compressed), &props, input.len() as u64)
                .unwrap()
        });
    });
    group.finish();
}

criterion_group!(benches, bench_decode);
criterion_main!(benches);
