use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use data_transformer_bench::{handlebars, hardcoded_serde, lua, rhai, rune, tera, vrl, Transform};
use serde_json::json;

fn transform_benchmark(c: &mut Criterion) {
    let test_value = json!({
        "name": "test",
        "value": 42,
        "nested": {
            "array": [1, 2, 3]
        }
    });

    let approaches: Vec<Box<dyn Transform>> = vec![
        Box::new(hardcoded_serde::Transformer),
        Box::new(handlebars::Transformer::default()),
        Box::new(tera::Transformer::default()),
        Box::new(vrl::Transformer::default()),
        Box::new(rhai::Transformer::default()),
        Box::new(lua::Transformer::default()),
        Box::new(rune::Transformer::default()),
    ];
    let mut group = c.benchmark_group("transform");
    for transform in data_transformer_bench::TRANSFORMS {
        for approach in approaches.iter() {
            if !approach.accept(transform) {
                continue;
            }
            group.bench_with_input(
                BenchmarkId::new(approach.name(), transform),
                transform,
                |b, transform| b.iter(|| approach.transform(transform, black_box(&test_value))),
            );
        }
    }
    group.finish();
}

criterion_group!(benches, transform_benchmark);
criterion_main!(benches);
