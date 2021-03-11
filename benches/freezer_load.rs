use criterion::{
    criterion_group, criterion_main, BenchmarkId, Criterion, SamplingMode, Throughput,
};
use mercury::freezer::BlockPart;
use std::ops::Range;
use std::path::PathBuf;

const START_BLOCK: u64 = 0;
const BLOCK_TEST_RANGE: Range<u64> = 1..11;

pub fn c_load_bodies(c: &mut Criterion) {
    dotenv::from_filename("bench.env").expect("Environment file bench.env not found");
    let ancient_folder = PathBuf::from(
        std::env::var("FREEZER_LOAD_BODIES")
            .expect("Environment variable FREEZER_LOAD_BODIES not found"),
    );

    let mut group = c.benchmark_group("c_load_bodies");
    for blocks in BLOCK_TEST_RANGE.map(|el| el * 100_000) {
        group.throughput(Throughput::Elements(blocks));
        group.sampling_mode(SamplingMode::Flat);
        group.sample_size(10);
        group.bench_with_input(
            BenchmarkId::from_parameter(blocks),
            &blocks,
            |bencher, &blocks| {
                bencher.iter(|| {
                    BlockPart::Bodies
                        .load(ancient_folder.as_path(), START_BLOCK, START_BLOCK + blocks)
                        .unwrap()
                })
            },
        );
    }
    group.finish()
}

criterion_group!(load_bodies, c_load_bodies);
criterion_main!(load_bodies);
