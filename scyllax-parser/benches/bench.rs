//! benches
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use scyllax_parser::{select::parse_select, SelectQuery};

fn select(query: String) -> SelectQuery {
    let res = parse_select(&query);
    res.unwrap().1
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("big", |b| {
        b.iter(|| {
            select(black_box(
				"select id, name, age from person where id = :id and name = :name and age > ? limit 10".to_string()
			))
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
