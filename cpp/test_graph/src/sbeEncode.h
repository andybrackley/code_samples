#pragma once

#include <array>

namespace SbeEncoding {
    std::array<char, 256> encodeBook();
    void decodeBook(std::array<char, 256>& buf);
}

