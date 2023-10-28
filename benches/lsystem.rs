use std::iter;

use criterion::{black_box, Criterion, criterion_group, criterion_main};
use criterion::BenchmarkId;
use criterion::Throughput;

use lsystem::{LSystemBuilder, LSystemError};
use lsystem::token::TokenId;

fn lsystem_algae(n: u64) -> Result<(), LSystemError> {
    let mut builder = LSystemBuilder::new();

    let a = builder.token("A")?;
    let b = builder.token("B")?;

    builder.axiom(vec![a])?;

    builder.production_rule(a, vec![a, b])?; // A -> AB
    builder.production_rule(b, vec![a])?;   // B -> A

    let mut system = builder.finish()?;

    system.step_by(n as usize);
    Ok(())
}
// fn rope_build_slice(n: u64) {
//     let mut rope = Rope::new();
//     for _ in 0..n {
//         let vec = Rope::from(vec![TokenId(0), 1.into(), 2.into(), 3.into(),0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into()]);
//         rope.append(vec)
//     }
// }

fn vec_append_slice(n: u64) {
    let mut vec = vec![];
    for _ in 0..n {
        let payload = vec![TokenId(0), 1.into(), 2.into(), 3.into(),0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into(), 0.into(), 1.into(), 2.into(), 3.into()];
        vec.extend(payload.iter().cloned())
    }
}

fn from_elem(c: &mut Criterion) {
    static KB: usize = 1024;

    let mut group = c.benchmark_group("from_elem");
    for size in [KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB].iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| iter::repeat(0u8).take(size).collect::<Vec<_>>());
        });
    }
    group.finish();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("lsystem algae 15", |b| b.iter(|| lsystem_algae(black_box(15))));
    // c.bench_function("rope build 256 x 264", |b| b.iter(|| rope_build_slice(black_box(256))));
    c.bench_function("vec append 256 x 264", |b| b.iter(|| vec_append_slice(black_box(256))));
}

criterion_group!(
    name = lsystem_bench;
    // config = Criterion::default().with_measurement(CyclesPerByte).with_plots();
    config = Criterion::default();
    targets = criterion_benchmark
);

criterion_main!(lsystem_bench);
