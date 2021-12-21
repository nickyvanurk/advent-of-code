use criterion::{black_box, criterion_group, criterion_main, Criterion};

use advent_of_code::*;

macro_rules! bench {
    ($c:expr, $path:path, $day_num:expr) => {{
        use $path::*;
        let s = read_input($day_num);
        $c.bench_function(concat!(stringify!($path), "::part1"), |b| {
            b.iter(|| black_box(part1(black_box(&s))))
        });
        $c.bench_function(concat!(stringify!($path), "::part2"), |b| {
            b.iter(|| black_box(part2(black_box(&s))))
        });
    }};
}

pub fn criterion_benchmark(c: &mut Criterion) {
    bench!(c, day01, 1);
    bench!(c, day02, 2);
    bench!(c, day03, 3);
    bench!(c, day04, 4);
    bench!(c, day04, 5);
    bench!(c, day06, 6);
    bench!(c, day07, 7);
    bench!(c, day08, 8);
    bench!(c, day09, 9);
    bench!(c, day10, 10);
    bench!(c, day11, 11);
    bench!(c, day12, 12);
    bench!(c, day13, 13);
    bench!(c, day14, 14);
    bench!(c, day15, 15);
    bench!(c, day16, 16);
    bench!(c, day17, 17);
    bench!(c, day18, 18);
    bench!(c, day19, 19);
    bench!(c, day20, 20);
    bench!(c, day21, 21);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
