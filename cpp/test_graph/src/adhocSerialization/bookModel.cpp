#include <iostream>

#include "bookModel.h"

namespace Adhoc {

const size_t headerSize = sizeof(size_t);

const size_t& serialize_sizeT(const BufferT* buffer, size_t& offset) {
    const size_t& value = buffer[offset += sizeof(size_t)];
    return value;
}

std::vector<BufferT> serialize(const Book& book) {
    std::vector<BufferT> buffer; 

    size_t nameLength = std::strlen(book.name) + 1; // +1 for the null terminator
    buffer.resize(sizeof(book.id) + sizeof(book.instrumentId) + sizeof(nameLength) + nameLength);
    size_t offset = 0;

    std::memcpy(buffer.data() + offset, &book.id, sizeof(book.id));
    offset += sizeof(book.id);

    std::memcpy(buffer.data() + offset, &book.instrumentId, sizeof(book.instrumentId));
    offset += sizeof(book.instrumentId);

    std::memcpy(buffer.data() + offset, &nameLength, sizeof(nameLength));
    offset += sizeof(nameLength);

    std::memcpy(buffer.data() + offset, book.name, nameLength);
    return buffer;
}

void deserialize_impl(const std::vector<uint8_t>& buffer, size_t& size, int32_t& id, uint32_t& instId, char*& name) {
    size_t offset = 0;

    // std::memcpy(&size, buffer.data() + offset, sizeof(size));
    // offset += sizeof(size);

    std::memcpy(&id, buffer.data() + offset, sizeof(id));
    offset += sizeof(id);

    std::memcpy(&instId, buffer.data() + offset, sizeof(instId));
    offset += sizeof(instId);

    size_t nameLength;
    std::memcpy(&nameLength, buffer.data() + offset, sizeof(nameLength));
    offset += sizeof(nameLength);

    name = new char[nameLength];
    std::memcpy(name, buffer.data() + offset, nameLength);
}

void deserialize_impl2(const std::vector<uint8_t>& buffer, size_t& size, int32_t& id, uint32_t& instId, const char*& name) {
    const uint8_t* data = buffer.data();
    size_t offset = 0;

    // size = *reinterpret_cast<const size_t*>(data + offset);
    // offset += sizeof(size);

    id = *reinterpret_cast<const int32_t*>(data + offset);
    offset += sizeof(id);

    instId = *reinterpret_cast<const uint64_t*>(data + offset);
    offset += sizeof(instId);

    size_t nameLength = *reinterpret_cast<const size_t*>(data + offset);
    offset += sizeof(nameLength);

    // name = new char[nameLength];
    // std::memcpy(name, data + offset, nameLength);
    name = reinterpret_cast<const char*>(data + offset);
}

void deserialize(const std::vector<uint8_t>& buffer) {
    const uint8_t* data = buffer.data();
    size_t offset = 0;

    Book b;

    deserialize_impl2(buffer, offset, b.id, b.instrumentId, b.name);

    std::cout << b.id << ":" << b.instrumentId << ":" << b.name << std::endl;
}

}
