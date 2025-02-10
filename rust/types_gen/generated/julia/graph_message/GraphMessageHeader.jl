struct GraphMessageHeader
    _id::GraphMessageId
    _parent_ids::Array{GraphMessageId}
    _ts_enqueued::Optional{Timestamp}
    _ts_in::Optional{Timestamp}
    _conflate_count::UInt64
    _msg_type::Optional{String}
    GraphMessageHeader(
        id::GraphMessageId,
        parent_ids::Array{GraphMessageId},
        ts_enqueued::Optional{Timestamp},
        ts_in::Optional{Timestamp},
        conflate_count::UInt64,
        msg_type::Optional{String},
    ) = new(
        id,
        parent_ids,
        ts_enqueued,
        ts_in,
        conflate_count,
        msg_type,
    )
end

const GraphMessageHeader_Reader = BufferedObj.Reader{ GraphMessageHeader }
# const BufferedObj.Writer{GraphMessageHeader} = BufferedObj.Writer{ GraphMessageHeader }
const GraphMessageHeaderT = Union{ GraphMessageHeader, BufferObjT{GraphMessageHeader} }

# Fields for GraphMessageHeader
id(x::GraphMessageHeader) = x._id
parent_ids(x::GraphMessageHeader) = x._parent_ids
ts_enqueued(x::GraphMessageHeader) = x._ts_enqueued
ts_in(x::GraphMessageHeader) = x._ts_in
conflate_count(x::GraphMessageHeader) = x._conflate_count
msg_type(x::GraphMessageHeader) = x._msg_type

# Fields for GraphMessageHeader_Reader
id(x::BufferObjT{GraphMessageHeader}) = GraphMessageHeaderImpl.id(x._buffer, x._start_pos, GraphMessageId_Reader)
parent_ids(x::BufferObjT{GraphMessageHeader}) = GraphMessageHeaderImpl.parent_ids(x._buffer, x._start_pos, BufferedArray.Instance{GraphMessageId_Reader})
ts_enqueued(x::BufferObjT{GraphMessageHeader}) = GraphMessageHeaderImpl.ts_enqueued(x._buffer, x._start_pos, Optional{Timestamp_Reader})
ts_in(x::BufferObjT{GraphMessageHeader}) = GraphMessageHeaderImpl.ts_in(x._buffer, x._start_pos, Optional{Timestamp_Reader})
conflate_count(x::BufferObjT{GraphMessageHeader}) = GraphMessageHeaderImpl.conflate_count(x._buffer, x._start_pos, UInt64)
msg_type(x::BufferObjT{GraphMessageHeader}) = GraphMessageHeaderImpl.msg_type(x._buffer, x._start_pos, Optional{String})

# Setters for BufferedObj.Writer{GraphMessageHeader}
function id!(x::BufferedObj.Writer{GraphMessageHeader}, val::GraphMessageId) 
    pos = BufferDirect.write!(x._buffer, x._start_pos + GraphMessageHeaderImpl.ID_OFFSET, val, GraphMessageId)
    x._write_pos[] = max(pos, x._write_pos[])
end
function conflate_count!(x::BufferedObj.Writer{GraphMessageHeader}, val::UInt64) 
    pos = BufferDirect.write!(x._buffer, x._start_pos + GraphMessageHeaderImpl.CONFLATE_COUNT_OFFSET, val, UInt64)
    x._write_pos[] = max(pos, x._write_pos[])
end
function parent_ids_writer(x::BufferedObj.Writer{GraphMessageHeader})
    x._write_pos[] = x._start_pos + GraphMessageHeaderImpl.PARENT_IDS_OFFSET
    return BufferedArray.Writer{BufferedObj.Writer{GraphMessageId}}(x._buffer, x._write_pos)
end
function ts_enqueued_writer(x::BufferedObj.Writer{GraphMessageHeader})
    GraphMessageHeaderImpl.set_actual_offset(x._buffer, x._start_pos, GraphMessageHeaderImpl.TS_ENQUEUED_INDEX, x._write_pos[])
    return BufferedObj.UnionWriter{Optional{BufferedObj.Writer{Timestamp}}}(x._buffer, x._write_pos)
end
function ts_in_writer(x::BufferedObj.Writer{GraphMessageHeader})
    GraphMessageHeaderImpl.set_actual_offset(x._buffer, x._start_pos, GraphMessageHeaderImpl.TS_IN_INDEX, x._write_pos[])
    return BufferedObj.UnionWriter{Optional{BufferedObj.Writer{Timestamp}}}(x._buffer, x._write_pos)
end
function msg_type_writer(x::BufferedObj.Writer{GraphMessageHeader})
    GraphMessageHeaderImpl.set_actual_offset(x._buffer, x._start_pos, GraphMessageHeaderImpl.MSG_TYPE_INDEX, x._write_pos[])
    return BufferedObj.UnionWriter{Optional{String}}(x._buffer, x._write_pos)
end
function finish(x::BufferedObj.Writer{GraphMessageHeader})
    # Store the size of the element 
    BufferDirect.write!(x._buffer, x._start_pos, x._write_pos[], Int64)
end 

# Fallback Equality operator when the specialized version don't match
function Base.:(==)(lhs::GraphMessageHeaderT, rhs::GraphMessageHeaderT)::Bool
    is_equal = true
    is_equal = is_equal && id(lhs) == id(rhs)
    is_equal = is_equal && conflate_count(lhs) == conflate_count(rhs)
    is_equal = is_equal && parent_ids(lhs) == parent_ids(rhs)
    is_equal = is_equal && ts_enqueued(lhs) == ts_enqueued(rhs)
    is_equal = is_equal && ts_in(lhs) == ts_in(rhs)
    is_equal = is_equal && msg_type(lhs) == msg_type(rhs)
    return is_equal
end

# Where the types are both buffer types we can simply do a memcmp
function Base.:(==)(lhs::BufferObjT{GraphMessageHeader}, rhs::BufferObjT{GraphMessageHeader})::Bool
    if lhs._start_pos == rhs._start_pos && lhs._buffer == rhs._buffer
        return true
    end

    lhs_size = GraphMessageHeaderImpl.get_elem_size(lhs._buffer, lhs._start_pos)
    rhs_size = GraphMessageHeaderImpl.get_elem_size(rhs._buffer, rhs._start_pos)
    if lhs_size != rhs_size
        return false
    end

# TODO: This should just be using Base.memcmp() but it wasn't being found when I tried to use it
    return tg_memcmp(pointer(lhs._buffer._buffer) + lhs._start_pos, pointer(rhs._buffer._buffer) + rhs._start_pos, lhs_size - 1)
end

module GraphMessageHeaderImpl

using ..Framework
import ..GraphMessageId
import ..Timestamp

import ..GraphMessageHeader
import ..GraphMessageHeader_Reader
# import ..BufferedObj.Writer{GraphMessageHeader}

# Offset Calculations
const OFFSET_COUNT = 3
const SLOT_COUNT = 4
const START_OFFSET = sizeof(Int64) * SLOT_COUNT
const ID_OFFSET::Int64 = START_OFFSET
const CONFLATE_COUNT_OFFSET::Int64 = ID_OFFSET + serialized_size(GraphMessageId)
const PARENT_IDS_OFFSET::Int64 = CONFLATE_COUNT_OFFSET + serialized_size(UInt64)
const END_FIXED_OFFSET::Int64 = CONFLATE_COUNT_OFFSET + serialized_size(UInt64)
const TS_ENQUEUED_INDEX = 2
const TS_IN_INDEX = 3
const MSG_TYPE_INDEX = 4

function id(buf::BufferDirect.Instance, start_pos::Int64, ::Type{T}) where {T}
    return BufferDirect.read(buf, Ref(start_pos + ID_OFFSET), T)
end
function conflate_count(buf::BufferDirect.Instance, start_pos::Int64, ::Type{T}) where {T}
    return BufferDirect.read(buf, Ref(start_pos + CONFLATE_COUNT_OFFSET), T)
end
function parent_ids(buf::BufferDirect.Instance, start_pos::Int64, ::Type{T}) where {T}
    return BufferDirect.read(buf, Ref(start_pos + PARENT_IDS_OFFSET), T)
end
function ts_enqueued(buf::BufferDirect.Instance, start_pos::Int64, ::Type{T}) where {T}
    index = TS_ENQUEUED_INDEX
    pos = get_actual_offset(buf, start_pos, index)
    return BufferDirect.read(buf, Ref(pos), T)
end
function ts_in(buf::BufferDirect.Instance, start_pos::Int64, ::Type{T}) where {T}
    index = TS_IN_INDEX
    pos = get_actual_offset(buf, start_pos, index)
    return BufferDirect.read(buf, Ref(pos), T)
end
function msg_type(buf::BufferDirect.Instance, start_pos::Int64, ::Type{T}) where {T}
    index = MSG_TYPE_INDEX
    pos = get_actual_offset(buf, start_pos, index)
    return BufferDirect.read(buf, Ref(pos), T)
end

# This gets the size of the struct when it was serialized to the buffer 
# i.e. end_pos - start_pos
# It works for both fixed and variable sized types however the implementations are different
@inline function get_elem_size(buf::BufferDirect.Instance, start_pos)::Int
    BufferDirect.read(buf, Ref(start_pos), Int64)
end
# Used to get the actual offset of variable length fields
#  ts_enqueued,  ts_in,  msg_type, 
@inline function get_actual_offset(buffer::BufferDirect.Instance, start_pos::Int64, index::Int64) 
    @assert index > 1 "get_actual_offset() index should be greater than 1, actual: $index"
    @assert index <= SLOT_COUNT "Index: $index exceeds offset count of $SLOT_COUNT"

    size_pos = start_pos + (index * sizeof(Int))
# @assert offset > END_FIXED_OFFSET "Offset $offset should be after the END_FIXED_OFFSET: $END_FIXED_OFFSET"
    return BufferDirect.read(buffer, Ref(size_pos), Int)
end

@inline function set_actual_offset(buffer::BufferDirect.Instance, start_pos::Int64, index::Int64, pos) 
    @assert index > 1 "set_actual_offset() index should be greater than 1, actual: $index"
    @assert index <= SLOT_COUNT "Index: $index exceeds offset count of $SLOT_COUNT"

    size_pos = start_pos + (index * sizeof(Int))
# @assert offset > END_FIXED_OFFSET "Offset $offset should be after the END_FIXED_OFFSET: $END_FIXED_OFFSET"
    return BufferDirect.write!(buffer, size_pos, pos, Int)
end

@inline function write_custom!(buf::BufferDirect.Instance, start_pos::Int64, x::GraphMessageHeader, GraphMessageHeader) 
    pos = start_pos + START_OFFSET
    pos = BufferDirect.write!(buf, pos, x._id, GraphMessageId)
    pos = BufferDirect.write!(buf, pos, x._conflate_count, UInt64)
    pos = BufferDirect.write!(buf, pos, x._parent_ids, Array{GraphMessageId})
    BufferDirect.write!(buf, start_pos + (TS_ENQUEUED_INDEX * sizeof(Int64)), pos, Int64 )
    pos = BufferDirect.write!(buf, pos, x._ts_enqueued, Optional{Timestamp})
    BufferDirect.write!(buf, start_pos + (TS_IN_INDEX * sizeof(Int64)), pos, Int64 )
    pos = BufferDirect.write!(buf, pos, x._ts_in, Optional{Timestamp})
    BufferDirect.write!(buf, start_pos + (MSG_TYPE_INDEX * sizeof(Int64)), pos, Int64 )
    pos = BufferDirect.write!(buf, pos, x._msg_type, Optional{String})
    # Store the serialized size of the element
    BufferDirect.write!(buf, start_pos, pos, Int64)
    return pos
end

@inline function read_custom(buf::BufferDirect.Instance, pos::Ref{Int}, ::Type{T}) where { T<:GraphMessageHeader }
    start_pos = pos[]
    pos[] += get_elem_size(buf, start_pos)

    GraphMessageHeader(
        id(buf, start_pos, GraphMessageId),
        parent_ids(buf, start_pos, Array{GraphMessageId}),
        ts_enqueued(buf, start_pos, Optional{Timestamp}),
        ts_in(buf, start_pos, Optional{Timestamp}),
        conflate_count(buf, start_pos, UInt64),
        msg_type(buf, start_pos, Optional{String}),
    )
end

# For the buffered version we simply wrap the buffer and start_pos
@inline function read_custom(buf::BufferDirect.Instance, pos::Ref{Int}, ::Type{T}) where { T<:BufferObjT{GraphMessageHeader} }
    inst = GraphMessageHeader_Reader(buf, pos[])
    pos[] += get_elem_size(buf, pos[])
    inst
end

function Framework.BufferDirect.buffer_to_value(x::BufferObjT{GraphMessageHeader})
    read_custom(x._buffer, Ref(x._start_pos), GraphMessageHeader)
end

end # End of module GraphMessageHeaderImpl

# Function Override Definitions
Framework.BufferDirect.get_elem_size(buf::BufferDirect.Instance, x::BufferObjT{GraphMessageHeader}) = GraphMessageHeaderImpl.get_elem_size(x._buffer, x._start_pos)
Framework.BufferDirect.write_custom!(buf::BufferDirect.Instance, pos::Int64, x::GraphMessageHeader, ::Type{ GraphMessageHeader }) = GraphMessageHeaderImpl.write_custom!(buf, pos, x, GraphMessageHeader)
Framework.BufferDirect.read_custom(buf::BufferDirect.Instance, pos::Ref{Int64}, ::Type{ GraphMessageHeader }) = GraphMessageHeaderImpl.read_custom(buf, pos, GraphMessageHeader)
Framework.BufferDirect.read_custom(buf::BufferDirect.Instance, pos::Ref{Int64}, ::Type{ T }) where { T<:BufferObjT{GraphMessageHeader} } = GraphMessageHeaderImpl.read_custom(buf, pos, BufferObjT{GraphMessageHeader})


