mutable struct GraphMessageId
    _id::UInt64
    GraphMessageId(
        id::UInt64,
    ) = new(
        id,
    )
end

const GraphMessageId_Reader = BufferedObj.Reader{ GraphMessageId }
# const BufferedObj.Writer{GraphMessageId} = BufferedObj.Writer{ GraphMessageId }
const GraphMessageIdT = Union{ GraphMessageId, BufferObjT{GraphMessageId} }

# Fields for GraphMessageId
id(x::GraphMessageId) = x._id
id!(x::GraphMessageId, value::UInt64) = x._id = value


# Fields for GraphMessageId_Reader
id(x::BufferObjT{GraphMessageId}) = GraphMessageIdImpl.id(x._buffer, x._start_pos, UInt64)

# Setters for BufferedObj.Writer{GraphMessageId}
function id!(x::BufferedObj.Writer{GraphMessageId}, val::UInt64) 
    pos = BufferDirect.write!(x._buffer, x._start_pos + GraphMessageIdImpl.ID_OFFSET, val, UInt64)
    x._write_pos[] = max(pos, x._write_pos[])
end 

# Fallback Equality operator when the specialized version don't match
function Base.:(==)(lhs::GraphMessageIdT, rhs::GraphMessageIdT)::Bool
    is_equal = true
    is_equal = is_equal && id(lhs) == id(rhs)
    return is_equal
end

# Where the types are both buffer types we can simply do a memcmp
function Base.:(==)(lhs::BufferObjT{GraphMessageId}, rhs::BufferObjT{GraphMessageId})::Bool
    if lhs._start_pos == rhs._start_pos && lhs._buffer == rhs._buffer
        return true
    end

    lhs_size = GraphMessageIdImpl.get_elem_size(lhs._buffer, lhs._start_pos)
    rhs_size = GraphMessageIdImpl.get_elem_size(rhs._buffer, rhs._start_pos)
    if lhs_size != rhs_size
        return false
    end

# TODO: This should just be using Base.memcmp() but it wasn't being found when I tried to use it
    return tg_memcmp(pointer(lhs._buffer._buffer) + lhs._start_pos, pointer(rhs._buffer._buffer) + rhs._start_pos, lhs_size - 1)
end

module GraphMessageIdImpl

using ..Framework

import ..GraphMessageId
import ..GraphMessageId_Reader
# import ..BufferedObj.Writer{GraphMessageId}

# Offset Calculations
const OFFSET_COUNT = 0
const SLOT_COUNT = 0
const START_OFFSET = sizeof(Int64) * SLOT_COUNT
const ID_OFFSET::Int64 = START_OFFSET
const END_FIXED_OFFSET::Int64 = ID_OFFSET + serialized_size(UInt64)

function id(buf::BufferDirect.Instance, start_pos::Int64, ::Type{T}) where {T}
    return BufferDirect.read(buf, Ref(start_pos + ID_OFFSET), T)
end
# This is basically a sizeof(T) but ignores any padding that may be added to a struct
# It's only available for fixed_size_types
@inline function Framework.serialized_size(::Type{T}) where {T<:GraphMessageId}
    END_FIXED_OFFSET
end

# This gets the size of the struct when it was serialized to the buffer 
# i.e. end_pos - start_pos
# It works for both fixed and variable sized types however the implementations are different
@inline function get_elem_size(buf::BufferDirect.Instance, start_pos::Int)::Int
    return serialized_size(GraphMessageId)
end
@inline function write_custom!(buf::BufferDirect.Instance, start_pos::Int64, x::GraphMessageId, GraphMessageId) 
    pos = start_pos + START_OFFSET
    pos = BufferDirect.write!(buf, pos, x._id, UInt64)
    return pos
end

@inline function read_custom(buf::BufferDirect.Instance, pos::Ref{Int}, ::Type{T}) where { T<:GraphMessageId }
    start_pos = pos[]
    pos[] += get_elem_size(buf, start_pos)

    GraphMessageId(
        id(buf, start_pos, UInt64),
    )
end

# For the buffered version we simply wrap the buffer and start_pos
@inline function read_custom(buf::BufferDirect.Instance, pos::Ref{Int}, ::Type{T}) where { T<:BufferObjT{GraphMessageId} }
    inst = GraphMessageId_Reader(buf, pos[])
    pos[] += get_elem_size(buf, pos[])
    inst
end

function Framework.BufferDirect.buffer_to_value(x::BufferObjT{GraphMessageId})
    read_custom(x._buffer, Ref(x._start_pos), GraphMessageId)
end

end # End of module GraphMessageIdImpl

# Function Override Definitions
Framework.BufferDirect.get_elem_size(buf::BufferDirect.Instance, x::BufferObjT{GraphMessageId}) = GraphMessageIdImpl.get_elem_size(x._buffer, x._start_pos)
Framework.BufferDirect.write_custom!(buf::BufferDirect.Instance, pos::Int64, x::GraphMessageId, ::Type{ GraphMessageId }) = GraphMessageIdImpl.write_custom!(buf, pos, x, GraphMessageId)
Framework.BufferDirect.read_custom(buf::BufferDirect.Instance, pos::Ref{Int64}, ::Type{ GraphMessageId }) = GraphMessageIdImpl.read_custom(buf, pos, GraphMessageId)
Framework.BufferDirect.read_custom(buf::BufferDirect.Instance, pos::Ref{Int64}, ::Type{ T }) where { T<:BufferObjT{GraphMessageId} } = GraphMessageIdImpl.read_custom(buf, pos, BufferObjT{GraphMessageId})


