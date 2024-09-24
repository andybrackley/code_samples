#pragma once

#include <array>
#include <vector>

namespace CapnProtoEncoding {
    std::vector<unsigned char> encodeBook();
    void decodeBook(std::vector<unsigned char>& buf);
}

