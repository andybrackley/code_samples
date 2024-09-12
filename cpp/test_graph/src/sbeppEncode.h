#pragma once

#include <array>

namespace SbeppEncoding {
    std::array<char, 256> encodeBook();
    void decodeBook(std::array<char, 256>& buf);
}

