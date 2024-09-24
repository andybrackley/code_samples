#pragma once

#include <vector>

namespace SbeEncoding {
    std::vector<char> encodeBook();
    void decodeBook(std::vector<char>& buf);
}

