use std::io::BufWriter;

use criterion::{ black_box, criterion_group, criterion_main, Criterion };

mod helpers;
use helpers::{
    get_book_update_test_obj,
    get_book_update_serializable_test_obj,
    get_default_buffer,
};

fn benchmark_serialize(c: &mut Criterion) {
    let mut buf: Vec<u8> = get_default_buffer();
    let obj = get_book_update_test_obj();

    c.bench_function("Ser::Ours", |b| {
        b.iter(|| {
            black_box(obj.serialize_into(&mut buf, 0));
        });
    });
}

fn benchmark_serialize_serde(c: &mut Criterion) {
    let mut buf: Vec<u8> = get_default_buffer();
    let obj = get_book_update_test_obj();

    c.bench_function("Ser::Serde", |b| {
        b.iter(|| {
            black_box(obj.serialize_into(&mut buf, 0));
        });
    });
}

fn benchmark_serialize_bincode(c: &mut Criterion) {
    let obj = get_book_update_serializable_test_obj();

    c.bench_function("Ser::BinCode", |b| {
        b.iter(|| {
            black_box(bincode::serialize(&obj)).unwrap();
        });
    });
}

fn benchmark_serialize_bincode_into(c: &mut Criterion) {
    let mut buffer = get_default_buffer();
    let obj = get_book_update_serializable_test_obj();
    let mut writer = BufWriter::new(&mut buffer);

    c.bench_function("Ser::BinCode_Into", |b| {
        b.iter(|| {
            black_box(bincode::serialize_into(&mut writer, &obj)).unwrap();
        });
    });
}

fn benchmark_serialize_bytemuck(c: &mut Criterion) {
    let mut buf: Vec<u8> = Vec::with_capacity(200);
    let obj = get_book_update_test_obj();

    c.bench_function("Ser::ByteMuck", |b| {
        b.iter(|| {
            black_box(obj.serialize_into(&mut buf, 0));
        });
    });
}

criterion_group!(
    benches,
    benchmark_serialize,
    // benchmark_serialize_serde,
    benchmark_serialize_bincode,
    benchmark_serialize_bincode_into
    // benchmark_serialize_bytemuck
);

criterion_main!(benches);
