#![allow(dead_code)]

// uncomment for unstable version
// #![allow(unused_attributes, incomplete_features)]
// #![feature(generic_const_exprs, const_for, const_mut_refs)]

use aoc_lib::AdventOfCode;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[path = "../src/main.rs"]
mod main;

fn bench_main(c: &mut Criterion) {
    c.bench_function("parse input", |b| {
        let input = include_str!("../input.txt");
        b.iter(|| main::Day10::parse_input(black_box(input)))
    });

    c.bench_function("solve 1", |b| {
        let input = main::Day10::parse_input(include_str!("../input.txt"));
        b.iter(|| main::Day10::solve_1(black_box(&input)))
    });

    c.bench_function("solve 2", |b| {
        let input = main::Day10::parse_input(include_str!("../input.txt"));
        b.iter(|| main::Day10::solve_2(black_box(&input)))
    });

    c.bench_function("parse sample input", |b| {
        let input = include_str!("../sample.txt");
        b.iter(|| main::Day10::parse_input(black_box(input)))
    });

    c.bench_function("solve 1 (sample input)", |b| {
        let input = main::Day10::parse_input(include_str!("../sample.txt"));
        b.iter(|| main::Day10::solve_1(black_box(&input)))
    });

    c.bench_function("solve 2 (sample input)", |b| {
        let input = main::Day10::parse_input(include_str!("../sample.txt"));
        b.iter(|| main::Day10::solve_2(black_box(&input)))
    });
}

criterion_group!(benches, bench_main);
criterion_main!(benches);
