//! string escape benches
//!
//! `cargo bench -p ryo3-json-ser --bench escape`
#![expect(unused_crate_dependencies)]
use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use ryo3_json_ser::escape::{escape_into_scalar, escape_into_swar_u64};

type EscapeFn = fn(&mut Vec<u8>, &str);

const FNS: [(&str, EscapeFn); 2] = [
    ("scalar", escape_into_scalar),
    ("swar", escape_into_swar_u64),
];

fn cases() -> Vec<(&'static str, Vec<String>)> {
    let keys = [
        "id",
        "url",
        "name",
        "text",
        "user_id",
        "created_at",
        "description",
    ];
    vec![
        (
            "keys_3-12b",
            keys.iter().map(|s| (*s).to_string()).collect(),
        ),
        ("clean_7b", vec!["babydog".to_string()]),
        ("clean_8b", vec!["babydogo".to_string()]),
        ("clean_20b", vec!["babydog_babydog_".to_string()]),
        (
            "clean_60b",
            vec!["The quick brown fox jumps over the baby dog, again and again".to_string()],
        ),
        ("clean_1kb", vec!["lorem ipsum dolor sit amet ".repeat(40)]),
        (
            "clean_64kb",
            vec!["lorem ipsum dolor sit amet ".repeat(2500)],
        ),
        ("utf8_1kb", vec!["日本語のテキスト héllo ".repeat(30)]),
        (
            "sparse_esc_1kb",
            vec!["some line of plain text here that ends\n".repeat(26)],
        ),
        ("dense_esc_200b", vec!["a\"b\\c\n".repeat(33)]),
        ("all_ctrl_200b", vec!["\u{1}".repeat(200)]),
    ]
}

fn bench_escape(c: &mut Criterion) {
    for (name, strs) in cases() {
        let total: usize = strs.iter().map(String::len).sum();
        let mut group = c.benchmark_group(format!("escape/{name}"));
        group.throughput(Throughput::Bytes(total as u64));
        for (fname, f) in FNS {
            group.bench_with_input(BenchmarkId::from_parameter(fname), &strs, |b, strs| {
                // worst case is 6 bytes out per byte in
                let mut out = Vec::with_capacity(total * 6);
                b.iter(|| {
                    out.clear();
                    for s in strs {
                        f(black_box(&mut out), black_box(s));
                    }
                    black_box(out.len())
                });
            });
        }
        group.finish();
    }
}

criterion_group!(benches, bench_escape);
criterion_main!(benches);
