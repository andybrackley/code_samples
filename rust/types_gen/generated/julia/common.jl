@enumx EdgeDirection begin
    In
    Out
end

struct Timestamp
    _stamp::UInt64

    Timestamp(
        stamp::UInt64,
    ) = new(
            stamp,
        )
end
function serialize(self::Timestamp, buf::Bytes, start_pos::Int64) 
    offsets = []
    pos = start_pos + sizeof(offsets)
    pos = serialize(buf, pos, self._stamp)
    serialize(buffer, start_pos, offsets)
    return pos
end

function deserialize(buf::Bytes, pos::Ref{Int}, ::Type{T}) where { T<:Timestamp }
    offsets = []
    pos[] += sizeof(offsets)
    Timestamp(
        deserialize(buf, pos, UInt64), # Deserialize stamp
    )
end
stamp(self::Timestamp) = self._stamp

struct Timestamp_Buffer
    buffer::Vector{UInt8}
    start_pos::UInt64
end

# Offset Calculations
const Timestamp_OFFSET_COUNT = 0
const Timestamp_START_OFFSET = sizeof(Int) * Timestamp_OFFSET_COUNT
const Timestamp_stamp_OFFSET = 0

function get_stamp(self::Timestamp_Buffer)
    return self._stamp
end

