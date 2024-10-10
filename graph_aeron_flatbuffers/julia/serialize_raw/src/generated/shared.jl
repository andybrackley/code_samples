include("../common.jl") 
include("../serialize.jl")
include("../deserialize.jl")

# TODO: I don't think this should be relying on anything outside this module
include("../../../src/messages/common.jl")


function serialize(stream::IO, id:: T) where { T <: IdTypes } 
    typename = string(T)
    serialize(stream, id.value)
end

function serialize(stream::IO, inst:: InstrumentId) 
    serialize(stream, inst.exchange)
    serialize(stream, inst.id)
 end

function deserialize(bytes:: Bytes, offset::Int32, ::Type{T}) where { T<:Timestamp}
    ptr = pointer(bytes) + offset
    value = unsafe_wrap(Array, ptr, sizeof(T))[1] 
    return Timestamp(value), (offset + sizeof(T))
end


function deserialize(bytes:: Bytes, offset::Int32, ::Type{T}) where { T<:InstrumentId}
    exchange, offset = deserialize(bytes, offset, Exchange) 
    id, offset = deserialize(bytes, Int32(offset), String)
    return InstrumentId(exchange, id), offset
end
