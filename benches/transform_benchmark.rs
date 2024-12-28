use criterion::{black_box, criterion_group, criterion_main, Criterion};
use data_transformer_bench::{handlebars, hardcoded_serde, tera, vrl, Transform};
use serde_json::json;

fn transform_benchmark(c: &mut Criterion) {
    let test_value = json!({
        "name": "test",
        "value": 42,
        "nested": {
            "array": [1, 2, 3]
        }
    });

    let approach1 = hardcoded_serde::Transformer::new();
    let approach2 = handlebars::Transformer::new();
    let approach3 = tera::Transformer::new();
    let approach4 = vrl::Transformer::new();
    for transform in data_transformer_bench::TRANSFORMS {
        c.bench_function(&format!("{}_hardcoded_serde", transform), |b| {
            b.iter(|| approach1.transform(black_box(transform), black_box(&test_value)))
        });
        c.bench_function(&format!("{}_handlebars", transform), |b| {
            b.iter(|| approach2.transform(black_box(transform), black_box(&test_value)))
        });
        c.bench_function(&format!("{}_tera", transform), |b| {
            b.iter(|| approach3.transform(black_box(transform), black_box(&test_value)))
        });
        c.bench_function(&format!("{}_vrl", transform), |b| {
            b.iter(|| approach4.transform(black_box(transform), black_box(&test_value)))
        });
    }
}

criterion_group!(benches, transform_benchmark);
criterion_main!(benches);
