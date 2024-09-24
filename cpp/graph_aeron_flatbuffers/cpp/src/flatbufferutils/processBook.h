#pragma once

#include <cstdint>
#include <vector>

#include "createBook.h"

namespace Graph {
namespace FlatBufferUtils {
    void FromBuffer(const std::vector<BufferT>& buf); 
}
}
