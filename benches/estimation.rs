//! Benchmarks for token estimation.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tokenx_rs::estimate_token_count;

fn bench_short_text(c: &mut Criterion) {
    let text = "The quick brown fox jumps over the lazy dog";
    c.bench_function("short_text (~9 words)", |b| {
        b.iter(|| estimate_token_count(black_box(text)))
    });
}

fn bench_medium_text(c: &mut Criterion) {
    let text = "The quick brown fox jumps over the lazy dog. ".repeat(100);
    c.bench_function("medium_text (~900 words)", |b| {
        b.iter(|| estimate_token_count(black_box(&text)))
    });
}

fn bench_long_text(c: &mut Criterion) {
    let text = "The quick brown fox jumps over the lazy dog. ".repeat(3000);
    c.bench_function("long_text (~27000 words)", |b| {
        b.iter(|| estimate_token_count(black_box(&text)))
    });
}

fn bench_cjk_text(c: &mut Criterion) {
    let text = "道可道非常道名可名非常名無名天地之始有名萬物之母".repeat(50);
    c.bench_function("cjk_text", |b| {
        b.iter(|| estimate_token_count(black_box(&text)))
    });
}

fn bench_code(c: &mut Criterion) {
    let text = r#"
fn process_items(items: &[Item]) -> Result<Vec<Output>, Error> {
    let mut results = Vec::with_capacity(items.len());
    for item in items {
        let output = item.transform()?;
        results.push(output);
    }
    Ok(results)
}
"#
    .repeat(100);
    c.bench_function("code_text", |b| {
        b.iter(|| estimate_token_count(black_box(&text)))
    });
}

criterion_group!(
    benches,
    bench_short_text,
    bench_medium_text,
    bench_long_text,
    bench_cjk_text,
    bench_code,
);
criterion_main!(benches);
