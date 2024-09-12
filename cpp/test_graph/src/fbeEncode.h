#pragma once

#include <array>
#include <vector>

namespace FbeEncoding {
    std::vector<uint8_t> encodeBook();
    void decodeBook(std::vector<uint8_t>& buf);
}

