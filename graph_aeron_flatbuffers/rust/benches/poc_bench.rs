use core::slice;
use std::borrow::Borrow;

use criterion::{ black_box, criterion_group, criterion_main, Criterion };

struct BookUpdateState {
    bids: Vec<i32>,
    asks: Vec<i32>,
}

enum State<'a> {
    FromState(BookUpdateState),
    FromBuffer(&'a [u8], usize),
}

struct BookUpdate<'a> {
    state: State<'a>,
}

impl<'a> BookUpdate<'a> {
    const ASK_OFFSET: usize = 5;

    pub fn from_state(state: BookUpdateState) -> BookUpdate<'a> {
        BookUpdate {
            state: State::FromState(state),
        }
    }

    pub fn from_buffer(buf: &'a [u8], start_pos: usize) -> BookUpdate<'a> {
        BookUpdate {
            state: State::FromBuffer(buf, start_pos),
        }
    }

    pub fn bids(&self) -> &[i32] {
        match &self.state {
            State::FromState(s) => { &s.bids }
            State::FromBuffer(buf, pos) => {
                unsafe { slice::from_raw_parts(buf.as_ptr() as *const i32, 1) }
            }
        }
    }

    pub fn asks(&self) -> &[i32] {
        match &self.state {
            State::FromState(s) => { &s.asks }
            State::FromBuffer(buf, pos) => {
                unsafe { slice::from_raw_parts(buf.as_ptr() as *const i32, 1) }
            }
        }
    }
}

fn benchmark_direct_deserialize(c: &mut Criterion) {
    c.bench_function("Using closures", |b| {
        b.iter(|| {});
    });
}

criterion_group!(benches, benchmark_direct_deserialize);

criterion_main!(benches);
