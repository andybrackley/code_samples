#include <string>

// Enum for BookUpdateType
enum class BookUpdateType : uint8_t {
    Update = 0,
    Snapshot = 1
};

// Enum for Exchange
enum class Exchange : uint16_t {
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
    Okx
};

// Optional type alias (using std::optional)
#include <optional>
using std::optional;

// Struct for Timestamp
struct Timestamp {
    int64_t value;
};

// Struct for Level
struct Level {
    int64_t value;
};

// Struct for InstrumentId
struct InstrumentId {
    Exchange exchange;
    std::string id;
};

