include("../serialize.jl")
include("../deserialize.jl")
include("../../../src/messages/book_update.jl")

include("shared.jl")

function serializeBookUpdate(stream::IO, obj::BookUpdate)
    serialize(stream, obj.time)

    # TODO: This isn't serializing as an Optional when an actual Timestamp is set.
    serialize(stream, obj.timestamp_exch) 
    serialize(stream, obj.instId)
    serialize(stream, obj.updateType)

    serialize(stream, obj.bids)
    serialize(stream, obj.asks)
    return position(stream)
end

function deserializeBookUpdate(bytes:: Bytes)  
    size, pos = deserialize(bytes, Int32(0), Int32)
    timestamp, pos = deserialize(bytes, Int32(pos), Timestamp)
    timestamp_exch, pos = deserialize(bytes, Int32(pos), Optional{Timestamp})
    instId, pos = deserialize(bytes, Int32(pos), InstrumentId)
    updateType, pos = deserialize(bytes, Int32(pos), BookUpdateType)

    bids, pos = deserialize(bytes, Int32(pos), Vector{Level})
    asks, pos = deserialize(bytes, Int32(pos), Vector{Level})
    
    bookUpdate = BookUpdate(timestamp, timestamp_exch, instId, updateType, bids, asks)
    return bookUpdate, pos
end