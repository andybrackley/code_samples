include("../serialize.jl")
include("../deserialize.jl")
include("../../../src/messages/book_update.jl")

include("shared.jl")

# TODO: 
#   I shouldn't need to do this here as I have the serialize defined 
#   in the "serialize.jl".  However if I call into the function defined there
#   it then fails to call into my "shared.jl" serialize functions so the
#   vectors aren't serialized correctly
function serialize(stream::IO, elements::Vector{T}) where {T}
    vector_len = length(elements)
    serialize(stream, vector_len)

    for element in elements
        serialize(stream, element)
    end
end

function serializeBookUpdate(stream::IO, obj::BookUpdate)
    serialize(stream, obj.time)
    # serialize(stream, obj.timestamp_exch)
    serialize(stream, obj.instId)
    serialize(stream, obj.updateType)

    serialize(stream, obj.bids)
    serialize(stream, obj.asks)
    return position(stream)
end

function deserializeBookUpdate(bytes:: Bytes)  
    size, pos = deserialize(bytes, Int32(0), Int32)
    timestamp, pos = deserialize(bytes, Int32(pos), Timestamp)
    # timestamp_exch, pos = deserialize(bytes, Int32(pos), Optional{Timestamp})
    instId, pos = deserialize(bytes, Int32(pos), InstrumentId)
    updateType, pos = deserialize(bytes, Int32(pos), BookUpdateType)

    bids, pos = deserialize(bytes, Int32(pos), Vector{Level})
    asks, pos = deserialize(bytes, Int32(pos), Vector{Level})
    
    bookUpdate = BookUpdate(timestamp, instId, updateType, bids, asks)
    return bookUpdate, pos
end