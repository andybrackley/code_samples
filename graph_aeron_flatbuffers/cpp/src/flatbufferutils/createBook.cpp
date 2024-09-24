#include "createBook.h"

#include "../../generated/book_generated.h"

namespace Graph {
namespace FlatBufferUtils {

std::vector<BufferT> AsBuffer() {
    flatbuffers::FlatBufferBuilder builder;

    auto instId = Graph::CreateInstrumentIdDirect(builder, Graph::Exchange::Exchange_Deribit, "InstId1");
    Graph::Timestamp timestamp(100);
    Graph::BookUpdateBuilder bub(builder);
    bub.add_timestamp(&timestamp); 
    bub.add_id(instId);
    auto offset = bub.Finish();
    Graph::FinishBookUpdateBuffer(builder, offset);

    auto bufferPointer = builder.GetBufferPointer();

    std::vector<BufferT> buf;
    buf.insert(buf.end(), bufferPointer, bufferPointer + builder.GetSize());
    return buf;
}

}
}