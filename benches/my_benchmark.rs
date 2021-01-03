use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc2020::{day1, day10, day11, day12, day2, day3, day6, day7, day8, day9};

fn criterion_benchmark(c: &mut Criterion) {
    //c.bench_function("day1part1", |b| b.iter(|| day1::tests::part1()));
    //c.bench_function("day1part2", |b| b.iter(|| day1::tests::part2()));
    //c.bench_function("day2part1", |b| b.iter(|| day2::tests::part1()));
    //c.bench_function("day2part2", |b| b.iter(|| day2::tests::part2()));
    //c.bench_function("day3part1", |b| b.iter(|| day3::tests::part1()));
    //c.bench_function("main", |b| b.iter(|| main::main()));
    //c.bench_function("main", |b| b.iter(|| day6::tests::part2()));
    //c.bench_function("main", |b| b.iter(|| day7::tests::part1()));
    //c.bench_function("main", |b| b.iter(|| day8::tests::part1()));
    //c.bench_function("main", |b| b.iter(|| day9::tests::part2()));
    //c.bench_function("main", |b| b.iter(|| day10::tests::part2()));
    //c.bench_function("main", |b| b.iter(|| day11::tests::part2()));
    //c.bench_function("main", |b| b.iter(|| day12::tests::part2()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
