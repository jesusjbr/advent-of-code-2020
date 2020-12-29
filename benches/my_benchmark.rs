use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc2020::{day1, day2, day3, main};

fn criterion_benchmark(c: &mut Criterion) {
    //c.bench_function("day1part1", |b| b.iter(|| day1::tests::part1()));
    //c.bench_function("day1part2", |b| b.iter(|| day1::tests::part2()));
    //c.bench_function("day2part1", |b| b.iter(|| day2::tests::part1()));
    //c.bench_function("day2part2", |b| b.iter(|| day2::tests::part2()));
    //c.bench_function("day3part1", |b| b.iter(|| day3::tests::part1()));
    c.bench_function("main", |b| b.iter(|| main::main()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
