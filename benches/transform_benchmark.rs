use criterion::{black_box, criterion_group, criterion_main, Criterion};
use data_transformer_bench::{CloneTransformer, HandlebarsTransformer};
use serde_json::json;

fn transform_benchmark(c: &mut Criterion) {
    let test_value = json!({
        "name": "test",
        "value": 42,
        "nested": {
            "array": [1, 2, 3]
        }
    });

    let approach1 = CloneTransformer::new();
    c.bench_function("clone", |b| {
        b.iter(|| approach1.transform(black_box("clone"), black_box(&test_value)))
    });

    let approach2 = HandlebarsTransformer::new();
    for transform in data_transformer_bench::TRANSFORMS {
        c.bench_function(&format!("{}_handlebars", transform), |b| {
            b.iter(|| approach2.transform(black_box(transform), black_box(&test_value)))
        });
    }
}

criterion_group!(benches, transform_benchmark);
criterion_main!(benches);
