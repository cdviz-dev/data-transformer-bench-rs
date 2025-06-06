use std::fs;

use criterion::{Criterion, criterion_group, criterion_main};
use data_transformer_bench::{Transform, handlebars, hardcoded_serde, lua, rhai, rune, tera, vrl};
use serde_json::json;
use std::hint::black_box;

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
    let transformations = data_transformer_bench::TRANSFORMS;
    // let transformations = vec![data_transformer_bench::GH_01];
    for transform in transformations {
        let mut group = c.benchmark_group(transform);
        for approach in approaches.iter() {
            if !approach.accept(transform) {
                continue;
            }
            let custom_value_path = format!("assets/values/{}.json", transform);
            let value = if fs::exists(&custom_value_path).expect("Failed to access path") {
                let txt = fs::read_to_string(custom_value_path).expect("Failed to read file");
                serde_json::from_str(&txt).expect("Failed to deserialize value")
            } else {
                test_value.clone()
            };
            group.bench_function(approach.name(), |b| {
                b.iter(|| approach.transform(black_box(transform), black_box(&value)))
            });
        }
        group.finish();
    }
}

criterion_group!(benches, transform_benchmark);
criterion_main!(benches);
