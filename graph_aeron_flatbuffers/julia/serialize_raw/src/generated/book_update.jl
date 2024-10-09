include("../serialize.jl")
include("../deserialize.jl")
include("../../../src/messages/book_update.jl")

include("shared.jl")

function serializeBookUpdate(stream::IO, obj::BookUpdate)
    serialize(stream, obj.time)
    serialize(stream, obj.timestamp_exch)
    serialize(stream, obj.instId)
    serialize(stream, obj.updateType)

    serialize(stream, obj.bids)
    serialize(stream, obj.asks)
    return position(stream)
end

function deserializeBookUpdate(bytes:: Bytes)  
    size, pos = deserialize(bytes, Int32(0), Int32)
    println("Deserialize expecting: $size bytes")

    timestamp, pos = deserialize(bytes, Int32(pos), Timestamp)
    println("Read::Timestamp::$timestamp")

    timestamp_exch, pos = deserialize(bytes, Int32(pos), Optional{Timestamp})
    println("Read::Timestamp::$timestamp_exch")

    instId, pos = deserialize(bytes, Int32(pos), InstrumentId)
    println("Read::InstId::$instId")

    updateType, pos = deserialize(bytes, Int32(pos), BookUpdateType)
    println("Read::UpdateType::$updateType")
    return pos
end