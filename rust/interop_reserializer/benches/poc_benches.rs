use criterion::{ black_box, criterion_group, criterion_main, Criterion };

mod helpers;
use helpers::{ get_default_buffer };
use generated_mod::poc_types::{ BookUpdatePoc, BookUpdatePocCreate, BookUpdatePocRead };

fn benchmark_direct_deserialize(c: &mut Criterion) {
    let mut buf: Vec<u8> = get_default_buffer();
    let bu = BookUpdatePocCreate { bids: vec![1, 2, 3, 4], asks: vec![9, 8, 7, 6] };
    let pos = bu.write_to_buffer(&mut buf, 0);
    unsafe {
        buf.set_len(pos);
    }

    c.bench_function("DeSer::PoC Direct", |b| {
        b.iter(|| {
            let buc = black_box(BookUpdatePocRead::from_buffer(&buf, 0));

            // The above doesn't really do anything.
            // For a valid test we need to actually retrieve some values.
            black_box(buc.bids());
            black_box(buc.asks());
        });
    });
}

criterion_group!(benches, benchmark_direct_deserialize);

criterion_main!(benches);
