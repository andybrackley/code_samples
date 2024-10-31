use criterion::{ black_box, criterion_group, criterion_main, Criterion };

mod helpers;
use generated_mod::types_tests::BookUpdate;
use helpers::{
    get_book_update_serializable_test_obj,
    get_book_update_test_obj,
    get_default_buffer,
    BookUpdateSerializable,
};

fn benchmark_deserialize(c: &mut Criterion) {
    let mut buf: Vec<u8> = get_default_buffer();
    let obj = get_book_update_test_obj();
    let pos = obj.serialize_into(&mut buf, 0);

    c.bench_function("DeSer::Ours", |b| {
        b.iter(|| {
            black_box(BookUpdate::deserialize_from(&buf, 0).unwrap());
        });
    });
}

fn benchmark_deserialize_serde(c: &mut Criterion) {
    let mut buf: Vec<u8> = get_default_buffer();
    let obj = get_book_update_test_obj();

    c.bench_function("DeSer::Serde", |b| {
        b.iter(|| {
            black_box(obj.serialize_into(&mut buf, 0));
        });
    });
}

fn benchmark_deserialize_bincode(c: &mut Criterion) {
    let obj = get_book_update_serializable_test_obj();
    let ser = bincode::serialize(&obj).unwrap();

    c.bench_function("DeSer::BinCode", |b| {
        b.iter(|| {
            black_box(bincode::deserialize::<BookUpdateSerializable>(&ser).unwrap());
        });
    });
}

fn benchmark_deserialize_bincode_into(c: &mut Criterion) {
    let mut buf = get_default_buffer();
    let obj = get_book_update_serializable_test_obj();

    let mut cursor = &buf[..];
    // let mut reader = BufReader::new(&mut buffer);

    //     c.bench_function("DeSer::BinCode_Into", |b| {
    //         b.iter(|| {
    //             let t: BookUpdateSerializable = bincode::deserialize_from(&mut cursor).unwrap();
    //             black_box(t);
    //         });
    //     });
}

fn benchmark_deserialize_bytemuck(c: &mut Criterion) {
    let mut buf: Vec<u8> = Vec::with_capacity(200);
    let obj = get_book_update_test_obj();

    c.bench_function("DeSer::ByteMuck", |b| {
        b.iter(|| {
            black_box(obj.serialize_into(&mut buf, 0));
        });
    });
}

criterion_group!(
    benches,
    benchmark_deserialize,
    // benchmark_serialize_serde,
    benchmark_deserialize_bincode,
    benchmark_deserialize_bincode_into
    // benchmark_serialize_bytemuck
);

criterion_main!(benches);