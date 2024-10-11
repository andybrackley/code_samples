pub mod shared;

// Enum for BookUpdateType
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BookUpdateType {
    Update = 0,
    Snapshot = 1,
}

// Enum for Exchange
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Exchange {
    Internal = 0,
    Binance,
    Bitstamp,
    Bitfinex,
    Bittrex,
    Coinbase,
    Deribit,
    Gateio,
    Gemini,
    Itbit,
    Kraken,
    Lmax,
    Okcoin,
    Okx,
}

// Generic type alias for Optional
type Optional<T> = Option<T>;

// Struct for Timestamp
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Timestamp {
    value: i64,
}

// Struct for Level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

struct Level {
    value: i64,
}

// Struct for InstrumentId
#[derive(Debug, Clone, PartialEq, Eq)]
struct InstrumentId {
    exchange: Exchange,
    id: String,
}
