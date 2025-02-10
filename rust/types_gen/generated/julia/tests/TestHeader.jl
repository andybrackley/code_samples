struct TestHeader
    _stamp::UInt64
    _parent_ids::Vector{GraphMessageId}
    _opt_id::Optional{GraphMessageId}
    _other_ids::Vector{GraphMessageId}
    TestHeader(
        stamp::UInt64,
        parent_ids::Vector{GraphMessageId},
        opt_id::Optional{GraphMessageId},
        other_ids::Vector{GraphMessageId},
    ) = new(
        stamp,
        parent_ids,
        opt_id,
        other_ids,
    )
end

struct TestHeader_Buffer <: BufferObj{ TestHeader }
    _buffer::BufferDirect.Instance
    _start_pos::Int64
    TestHeader_Buffer(buffer, start_pos)  = new(buffer, start_pos)
end

const TestHeaderT = Union{ TestHeader, TestHeader_Buffer }

# Fallback Equality operator when the specialized version don't match
function Base.:(==)(lhs::TestHeaderT, rhs::TestHeaderT)::Bool
    is_equal = true
    is_equal = is_equal && stamp(lhs) == stamp(rhs)
    is_equal = is_equal && parent_ids(lhs) == parent_ids(rhs)
    is_equal = is_equal && opt_id(lhs) == opt_id(rhs)
    is_equal = is_equal && other_ids(lhs) == other_ids(rhs)
    return is_equal
end

# Where the types are both buffer types we can simply do a memcmp
function Base.:(==)(lhs::TestHeader_Buffer, rhs::TestHeader_Buffer)::Bool
    if lhs._start_pos == rhs._start_pos && lhs._buffer == rhs._buffer
        return true
    end

    lhs_size = TestHeaderImpl.get_elem_size(lhs._buffer, lhs._start_pos)
    rhs_size = TestHeaderImpl.get_elem_size(rhs._buffer, rhs._start_pos)
    if lhs_size != rhs_size
        return false
    end

# TODO: This should just be using Base.memcmp() but it wasn't being found when I tried to use it
    return tg_memcmp(pointer(lhs._buffer._buffer) + lhs._start_pos, pointer(rhs._buffer._buffer) + rhs._start_pos, lhs_size - 1)
end

module TestHeaderImpl

using ..Framework
import ..GraphMessageId

import ..TestHeader
import ..TestHeader_Buffer

# Offset Calculations
const OFFSET_COUNT = 2

const SLOT_COUNT = 3 + OFFSET_COUNT
const START_OFFSET = sizeof(Int64) * SLOT_COUNT
const STAMP_OFFSET::Int64 = START_OFFSET
const PARENT_IDS_OFFSET::Int64 = STAMP_OFFSET + serialized_size(UInt64)
const END_FIXED_OFFSET::Int64 = STAMP_OFFSET + serialized_size(UInt64)
const OPT_ID_INDEX = 2
const OTHER_IDS_INDEX = 3

function stamp(buf::BufferDirect.Instance, start_pos::Int64, ::Type{T}) where {T}
    return BufferDirect.read(buf, Ref(start_pos + STAMP_OFFSET), T)
end
function parent_ids(buf::BufferDirect.Instance, start_pos::Int64, ::Type{T}) where {T}
    return BufferDirect.read(buf, Ref(start_pos + PARENT_IDS_OFFSET), T)
end

# This gets the size of the struct when it was serialized to the buffer 
# i.e. end_pos - start_pos
# It works for both fixed and variable sized types however the implementations are different
@inline function get_elem_size(buf::BufferDirect.Instance, start_pos)::Int
    BufferDirect.read(buf, Ref(start_pos), Int64)
end
# Used to get the actual offset of variable length fields
#  opt_id,  other_ids, 
@inline function get_actual_offset(x::TestHeader_Buffer, index::Int64) 
    @assert index > 1 "get_actual_offset() index should be greater than 1, actual: $index"
    @assert index <= TestHeader_SLOT_COUNT "Index: $index exceeds offset count of $TestHeader_SLOT_COUNT"

    offset = x._start_pos + (index * sizeof(Int))
    @assert offset > END_FIXED_OFFSET "Offset $offset should be after the END_FIXED_OFFSET: $END_FIXED_OFFSET"
    return BufferDirect.read(x._buffer, Ref(offset), Int)
end


@inline function write_custom!(buf::BufferDirect.Instance, start_pos::Int64, x::TestHeader, TestHeader) 
    pos = start_pos
    pos = BufferDirect.write!(buf, pos, x._stamp, UInt64)
    pos = BufferDirect.write!(buf, pos, x._parent_ids, Vector{GraphMessageId})
    pos = BufferDirect.write!(buf, pos, x._opt_id, Optional{GraphMessageId})
    pos = BufferDirect.write!(buf, pos, x._other_ids, Vector{GraphMessageId})
    return pos
end

@inline function read_custom(buf::BufferDirect.Instance, pos::Ref{Int}, ::Type{T}) where { T<:TestHeader }
    start_pos = pos[]
    pos[] += get_elem_size(buf, start_pos)

    TestHeader(
        stamp(buf, start_pos, UInt64),
        parent_ids(buf, start_pos, Vector{GraphMessageId}),
        opt_id(buf, start_pos, Optional{GraphMessageId}),
        other_ids(buf, start_pos, Vector{GraphMessageId}),
    )
end

# For the buffered version we simply wrap the buffer and start_pos
@inline function read_custom(buf::BufferDirect.Instance, pos::Ref{Int}, ::Type{T}) where { T<:TestHeader_Buffer }
    inst = TestHeader_Buffer(buf, pos[])
    pos[] += get_elem_size(buf, pos[])
    inst
end

function Framework.BufferDirect.buffer_to_value(x::TestHeader_Buffer)
    read_custom(x._buffer, Ref(x._start_pos), TestHeader)
end

end # End of module TestHeaderImpl

# Fields for TestHeader
stamp(x::TestHeader) = x._stamp
parent_ids(x::TestHeader) = x._parent_ids
opt_id(x::TestHeader) = x._opt_id
other_ids(x::TestHeader) = x._other_ids

# Fields for TestHeader_Buffer
stamp(x::TestHeader_Buffer) = TestHeaderImpl.stamp(x._buffer, x._start_pos, UInt64)
parent_ids(x::TestHeader_Buffer) = TestHeaderImpl.parent_ids(x._buffer, x._start_pos, Vector{GraphMessageId})
opt_id(x::TestHeader_Buffer) = TestHeaderImpl.opt_id(x._buffer, x._start_pos, Optional{GraphMessageId})
other_ids(x::TestHeader_Buffer) = TestHeaderImpl.other_ids(x._buffer, x._start_pos, Vector{GraphMessageId})

# Function Override Definitions

Framework.BufferDirect.get_elem_size(buf::BufferDirect.Instance, x::TestHeader_Buffer) = TestHeaderImpl.get_elem_size(x._buffer, x._start_pos)
Framework.BufferDirect.write_custom!(buf::BufferDirect.Instance, pos::Int64, x::TestHeader, ::Type{ TestHeader }) = TestHeaderImpl.write_custom!(buf, pos, x, TestHeader)
Framework.BufferDirect.read_custom(buf::BufferDirect.Instance, pos::Ref{Int64}, ::Type{ TestHeader }) = TestHeaderImpl.read_custom(buf, pos, TestHeader)
Framework.BufferDirect.read_custom(buf::BufferDirect.Instance, pos::Ref{Int64}, ::Type{ TestHeader_Buffer }) = TestHeaderImpl.read_custom(buf, pos, TestHeader_Buffer)


