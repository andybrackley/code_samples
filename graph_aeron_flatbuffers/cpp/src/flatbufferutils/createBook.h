#pragma once

#include <cstdint>
#include <vector>

namespace Graph {
namespace FlatBufferUtils
{
    using BufferT = uint8_t;
    std::vector<BufferT> AsBuffer();
}
}
