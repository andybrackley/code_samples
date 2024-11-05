use criterion::{ black_box, criterion_group, criterion_main, Criterion };

mod helpers;
use generated_mod::types_tests_serialization::BookUpdateBuffer;
use helpers::{ get_book_update_test_obj, get_default_buffer };

fn benchmark_direct_deserialize(c: &mut Criterion) {
    let mut buf: Vec<u8> = get_default_buffer();
    let bu = get_book_update_test_obj();
    let pos = bu.serialize_into(&mut buf, 0);
    unsafe {
        let _ = buf.align_to::<i8>();
        buf.set_len(pos);
    }

    c.bench_function("DeSer::PoC Direct", |b| {
        b.iter(|| {
            let buc = black_box(BookUpdateBuffer::from_buffer(&buf, 0));

            // The above doesn't really do anything.
            // For a valid test we need to actually retrieve some values.
            black_box(buc.bids());
            black_box(buc.asks());
        });
    });
}

criterion_group!(benches, benchmark_direct_deserialize);

criterion_main!(benches);
