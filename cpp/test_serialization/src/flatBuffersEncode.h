#include <vector>

namespace FlatBuffersEncode {
    std::vector<uint8_t> encodeBook();
    void decodeBook(std::vector<uint8_t>& buf);
}
