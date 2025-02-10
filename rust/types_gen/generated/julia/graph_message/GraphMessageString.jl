struct GraphMessageString
    _header::GraphMessageHeader
    _data::String
    GraphMessageString(
        header::GraphMessageHeader,
        data::String,
    ) = new(
        header,
        data,
    )
end

const GraphMessageString_Reader = BufferedObj.Reader{ GraphMessageString }
# const BufferedObj.Writer{GraphMessageString} = BufferedObj.Writer{ GraphMessageString }
const GraphMessageStringT = Union{ GraphMessageString, BufferObjT{GraphMessageString} }

# Fields for GraphMessageString
header(x::GraphMessageString) = x._header
data(x::GraphMessageString) = x._data

# Fields for GraphMessageString_Reader
header(x::BufferObjT{GraphMessageString}) = GraphMessageStringImpl.header(x._buffer, x._start_pos, GraphMessageHeader_Reader)
data(x::BufferObjT{GraphMessageString}) = GraphMessageStringImpl.data(x._buffer, x._start_pos, String)

# Setters for BufferedObj.Writer{GraphMessageString}
function header_writer(x::BufferedObj.Writer{GraphMessageString})
    x._write_pos[] = x._start_pos + GraphMessageStringImpl.HEADER_OFFSET
    return BufferedObj.Writer{GraphMessageHeader}(x._buffer, x._write_pos)
end
function data_writer(x::BufferedObj.Writer{GraphMessageString})
    GraphMessageStringImpl.set_actual_offset(x._buffer, x._start_pos, GraphMessageStringImpl.DATA_INDEX, x._write_pos[])
    return BufferedObj.Writer{String}(x._buffer, x._write_pos)
end
function finish(x::BufferedObj.Writer{GraphMessageString})
    # Store the size of the element 
    BufferDirect.write!(x._buffer, x._start_pos, x._write_pos[], Int64)
end 

# Fallback Equality operator when the specialized version don't match
function Base.:(==)(lhs::GraphMessageStringT, rhs::GraphMessageStringT)::Bool
    is_equal = true
    is_equal = is_equal && header(lhs) == header(rhs)
    is_equal = is_equal && data(lhs) == data(rhs)
    return is_equal
end

# Where the types are both buffer types we can simply do a memcmp
function Base.:(==)(lhs::BufferObjT{GraphMessageString}, rhs::BufferObjT{GraphMessageString})::Bool
    if lhs._start_pos == rhs._start_pos && lhs._buffer == rhs._buffer
        return true
    end

    lhs_size = GraphMessageStringImpl.get_elem_size(lhs._buffer, lhs._start_pos)
    rhs_size = GraphMessageStringImpl.get_elem_size(rhs._buffer, rhs._start_pos)
    if lhs_size != rhs_size
        return false
    end

# TODO: This should just be using Base.memcmp() but it wasn't being found when I tried to use it
    return tg_memcmp(pointer(lhs._buffer._buffer) + lhs._start_pos, pointer(rhs._buffer._buffer) + rhs._start_pos, lhs_size - 1)
end

module GraphMessageStringImpl

using ..Framework
import ..GraphMessageHeader

import ..GraphMessageString
import ..GraphMessageString_Reader
# import ..BufferedObj.Writer{GraphMessageString}

# Offset Calculations
const OFFSET_COUNT = 1
const SLOT_COUNT = 2
const START_OFFSET = sizeof(Int64) * SLOT_COUNT
const HEADER_OFFSET::Int64 = START_OFFSET
const END_FIXED_OFFSET::Int64 = START_OFFSET
const DATA_INDEX = 2

function header(buf::BufferDirect.Instance, start_pos::Int64, ::Type{T}) where {T}
    return BufferDirect.read(buf, Ref(start_pos + HEADER_OFFSET), T)
end
function data(buf::BufferDirect.Instance, start_pos::Int64, ::Type{T}) where {T}
    index = DATA_INDEX
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
#  data, 
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

@inline function write_custom!(buf::BufferDirect.Instance, start_pos::Int64, x::GraphMessageString, GraphMessageString) 
    pos = start_pos + START_OFFSET
    pos = BufferDirect.write!(buf, pos, x._header, GraphMessageHeader)
    BufferDirect.write!(buf, start_pos + (DATA_INDEX * sizeof(Int64)), pos, Int64 )
    pos = BufferDirect.write!(buf, pos, x._data, String)
    # Store the serialized size of the element
    BufferDirect.write!(buf, start_pos, pos, Int64)
    return pos
end

@inline function read_custom(buf::BufferDirect.Instance, pos::Ref{Int}, ::Type{T}) where { T<:GraphMessageString }
    start_pos = pos[]
    pos[] += get_elem_size(buf, start_pos)

    GraphMessageString(
        header(buf, start_pos, GraphMessageHeader),
        data(buf, start_pos, String),
    )
end

# For the buffered version we simply wrap the buffer and start_pos
@inline function read_custom(buf::BufferDirect.Instance, pos::Ref{Int}, ::Type{T}) where { T<:BufferObjT{GraphMessageString} }
    inst = GraphMessageString_Reader(buf, pos[])
    pos[] += get_elem_size(buf, pos[])
    inst
end

function Framework.BufferDirect.buffer_to_value(x::BufferObjT{GraphMessageString})
    read_custom(x._buffer, Ref(x._start_pos), GraphMessageString)
end

end # End of module GraphMessageStringImpl

# Function Override Definitions
Framework.BufferDirect.get_elem_size(buf::BufferDirect.Instance, x::BufferObjT{GraphMessageString}) = GraphMessageStringImpl.get_elem_size(x._buffer, x._start_pos)
Framework.BufferDirect.write_custom!(buf::BufferDirect.Instance, pos::Int64, x::GraphMessageString, ::Type{ GraphMessageString }) = GraphMessageStringImpl.write_custom!(buf, pos, x, GraphMessageString)
Framework.BufferDirect.read_custom(buf::BufferDirect.Instance, pos::Ref{Int64}, ::Type{ GraphMessageString }) = GraphMessageStringImpl.read_custom(buf, pos, GraphMessageString)
Framework.BufferDirect.read_custom(buf::BufferDirect.Instance, pos::Ref{Int64}, ::Type{ T }) where { T<:BufferObjT{GraphMessageString} } = GraphMessageStringImpl.read_custom(buf, pos, BufferObjT{GraphMessageString})


