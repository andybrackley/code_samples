#include <optional>
#include <vector>

#include "shared.hpp"

// Struct for BookUpdate
struct BookUpdate {
    Timestamp time;
    std::optional<Timestamp> timestamp_exch;
    InstrumentId inst_id;
    BookUpdateType update_type;
    std::vector<Level> bids;
    std::vector<Level> asks;
};

// Struct for BookUpdateFull
struct BookUpdateFull {
    Timestamp time;
    std::optional<Timestamp> timestamp_exch;
    InstrumentId inst_id;
    std::vector<Level> bids;
    std::vector<Level> asks;
};
