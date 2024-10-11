mod shared;
use shared::{Timestamp, InstrumentId, BookUpdateType, Level, Exchange};

// Struct for BookUpdate
#[derive(Debug, Clone, PartialEq)]
struct BookUpdate {
    time: Timestamp,
    timestamp_exch: Optional<Timestamp>,
    inst_id: InstrumentId,
    update_type: BookUpdateType,
    bids: Vec<Level>,
    asks: Vec<Level>,
}

// Struct for BookUpdateFull
#[derive(Debug, Clone, PartialEq)]
struct BookUpdateFull {
    time: Timestamp,
    timestamp_exch: Optional<Timestamp>,
    inst_id: InstrumentId,
    bids: Vec<Level>,
    asks: Vec<Level>,
}
