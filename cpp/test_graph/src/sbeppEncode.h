#pragma once

#include <vector>

namespace SbeppEncoding {
    std::vector<char> encodeBook();
    void decodeBook(std::vector<char>& buf);
}

