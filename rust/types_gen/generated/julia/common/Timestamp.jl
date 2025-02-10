struct Timestamp
    _stamp::UInt64
    Timestamp(
        stamp::UInt64,
    ) = new(
        stamp,
    )
end

const Timestamp_Reader = BufferedObj.Reader{ Timestamp }
# const BufferedObj.Writer{Timestamp} = BufferedObj.Writer{ Timestamp }
const TimestampT = Union{ Timestamp, BufferObjT{Timestamp} }

# Fields for Timestamp
stamp(x::Timestamp) = x._stamp

# Fields for Timestamp_Reader
stamp(x::BufferObjT{Timestamp}) = TimestampImpl.stamp(x._buffer, x._start_pos, UInt64)

# Setters for BufferedObj.Writer{Timestamp}
function stamp!(x::BufferedObj.Writer{Timestamp}, val::UInt64) 
    pos = BufferDirect.write!(x._buffer, x._start_pos + TimestampImpl.STAMP_OFFSET, val, UInt64)
    x._write_pos[] = max(pos, x._write_pos[])
end 

# Fallback Equality operator when the specialized version don't match
function Base.:(==)(lhs::TimestampT, rhs::TimestampT)::Bool
    is_equal = true
    is_equal = is_equal && stamp(lhs) == stamp(rhs)
    return is_equal
end

# Where the types are both buffer types we can simply do a memcmp
function Base.:(==)(lhs::BufferObjT{Timestamp}, rhs::BufferObjT{Timestamp})::Bool
    if lhs._start_pos == rhs._start_pos && lhs._buffer == rhs._buffer
        return true
    end

    lhs_size = TimestampImpl.get_elem_size(lhs._buffer, lhs._start_pos)
    rhs_size = TimestampImpl.get_elem_size(rhs._buffer, rhs._start_pos)
    if lhs_size != rhs_size
        return false
    end

# TODO: This should just be using Base.memcmp() but it wasn't being found when I tried to use it
    return tg_memcmp(pointer(lhs._buffer._buffer) + lhs._start_pos, pointer(rhs._buffer._buffer) + rhs._start_pos, lhs_size - 1)
end

module TimestampImpl

using ..Framework

import ..Timestamp
import ..Timestamp_Reader
# import ..BufferedObj.Writer{Timestamp}

# Offset Calculations
const OFFSET_COUNT = 0
const SLOT_COUNT = 0
const START_OFFSET = sizeof(Int64) * SLOT_COUNT
const STAMP_OFFSET::Int64 = START_OFFSET
const END_FIXED_OFFSET::Int64 = STAMP_OFFSET + serialized_size(UInt64)

function stamp(buf::BufferDirect.Instance, start_pos::Int64, ::Type{T}) where {T}
    return BufferDirect.read(buf, Ref(start_pos + STAMP_OFFSET), T)
end
# This is basically a sizeof(T) but ignores any padding that may be added to a struct
# It's only available for fixed_size_types
@inline function Framework.serialized_size(::Type{T}) where {T<:Timestamp}
    END_FIXED_OFFSET
end

# This gets the size of the struct when it was serialized to the buffer 
# i.e. end_pos - start_pos
# It works for both fixed and variable sized types however the implementations are different
@inline function get_elem_size(buf::BufferDirect.Instance, start_pos::Int)::Int
    return serialized_size(Timestamp)
end
@inline function write_custom!(buf::BufferDirect.Instance, start_pos::Int64, x::Timestamp, Timestamp) 
    pos = start_pos + START_OFFSET
    pos = BufferDirect.write!(buf, pos, x._stamp, UInt64)
    return pos
end

@inline function read_custom(buf::BufferDirect.Instance, pos::Ref{Int}, ::Type{T}) where { T<:Timestamp }
    start_pos = pos[]
    pos[] += get_elem_size(buf, start_pos)

    Timestamp(
        stamp(buf, start_pos, UInt64),
    )
end

# For the buffered version we simply wrap the buffer and start_pos
@inline function read_custom(buf::BufferDirect.Instance, pos::Ref{Int}, ::Type{T}) where { T<:BufferObjT{Timestamp} }
    inst = Timestamp_Reader(buf, pos[])
    pos[] += get_elem_size(buf, pos[])
    inst
end

function Framework.BufferDirect.buffer_to_value(x::BufferObjT{Timestamp})
    read_custom(x._buffer, Ref(x._start_pos), Timestamp)
end

end # End of module TimestampImpl

# Function Override Definitions
Framework.BufferDirect.get_elem_size(buf::BufferDirect.Instance, x::BufferObjT{Timestamp}) = TimestampImpl.get_elem_size(x._buffer, x._start_pos)
Framework.BufferDirect.write_custom!(buf::BufferDirect.Instance, pos::Int64, x::Timestamp, ::Type{ Timestamp }) = TimestampImpl.write_custom!(buf, pos, x, Timestamp)
Framework.BufferDirect.read_custom(buf::BufferDirect.Instance, pos::Ref{Int64}, ::Type{ Timestamp }) = TimestampImpl.read_custom(buf, pos, Timestamp)
Framework.BufferDirect.read_custom(buf::BufferDirect.Instance, pos::Ref{Int64}, ::Type{ T }) where { T<:BufferObjT{Timestamp} } = TimestampImpl.read_custom(buf, pos, BufferObjT{Timestamp})


