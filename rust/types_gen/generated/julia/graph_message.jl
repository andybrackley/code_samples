mutable struct GraphMessageId
    _id::UInt64

    GraphMessageId(
        id::UInt64,
    ) = new(
            id,
        )
end
function serialize(self::GraphMessageId, buf::Bytes, start_pos::Int64) 
    offsets = []
    pos = start_pos + sizeof(offsets)
    pos = serialize(buf, pos, self._id)
    serialize(buffer, start_pos, offsets)
    return pos
end

function deserialize(buf::Bytes, pos::Ref{Int}, ::Type{T}) where { T<:GraphMessageId }
    offsets = []
    pos[] += sizeof(offsets)
    GraphMessageId(
        deserialize(buf, pos, UInt64), # Deserialize id
    )
end
id(self::GraphMessageId) = self._id 
id!(self::GraphMessageId, value::UInt64) = self._id = value

struct GraphMessageId_Buffer
    buffer::Vector{UInt8}
    start_pos::UInt64
end

# Offset Calculations
const GraphMessageId_OFFSET_COUNT = 0
const GraphMessageId_START_OFFSET = sizeof(Int) * GraphMessageId_OFFSET_COUNT
const GraphMessageId_id_OFFSET = 0

function get_id(self::GraphMessageId_Buffer)
    return self._id
end

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
function serialize(self::GraphMessageHeader, buf::Bytes, start_pos::Int64) 
    offsets = []
    pos = start_pos + sizeof(offsets)
    pos = serialize(buf, pos, self._id)
    pos = serialize(buf, pos, self._parent_ids)
    pos = serialize(buf, pos, self._ts_enqueued)
    pos = serialize(buf, pos, self._ts_in)
    pos = serialize(buf, pos, self._conflate_count)
    pos = serialize(buf, pos, self._msg_type)
    serialize(buffer, start_pos, offsets)
    return pos
end

function deserialize(buf::Bytes, pos::Ref{Int}, ::Type{T}) where { T<:GraphMessageHeader }
    offsets = []
    pos[] += sizeof(offsets)
    GraphMessageHeader(
        deserialize(buf, pos, GraphMessageId), # Deserialize id
        deserialize(buf, pos, Array{GraphMessageId}), # Deserialize parent_ids
        deserialize(buf, pos, Optional{Timestamp}), # Deserialize ts_enqueued
        deserialize(buf, pos, Optional{Timestamp}), # Deserialize ts_in
        deserialize(buf, pos, UInt64), # Deserialize conflate_count
        deserialize(buf, pos, Optional{String}), # Deserialize msg_type
    )
end
id(self::GraphMessageHeader) = self._id
parent_ids(self::GraphMessageHeader) = self._parent_ids
ts_enqueued(self::GraphMessageHeader) = self._ts_enqueued
ts_in(self::GraphMessageHeader) = self._ts_in
conflate_count(self::GraphMessageHeader) = self._conflate_count
msg_type(self::GraphMessageHeader) = self._msg_type

struct GraphMessageHeader_Buffer
    buffer::Vector{UInt8}
    start_pos::UInt64
end

# Offset Calculations
const GraphMessageHeader_OFFSET_COUNT = 0
const GraphMessageHeader_START_OFFSET = sizeof(Int) * GraphMessageHeader_OFFSET_COUNT
const GraphMessageHeader_id_OFFSET = 0
const GraphMessageHeader_parent_ids_OFFSET = 0
const GraphMessageHeader_ts_enqueued_OFFSET = 0
const GraphMessageHeader_ts_in_OFFSET = 0
const GraphMessageHeader_conflate_count_OFFSET = 0
const GraphMessageHeader_msg_type_OFFSET = 0

function get_id(self::GraphMessageHeader_Buffer)
    return self._id
end

function get_parent_ids(self::GraphMessageHeader_Buffer)
    return self._parent_ids
end

function get_ts_enqueued(self::GraphMessageHeader_Buffer)
    return self._ts_enqueued
end

function get_ts_in(self::GraphMessageHeader_Buffer)
    return self._ts_in
end

function get_conflate_count(self::GraphMessageHeader_Buffer)
    return self._conflate_count
end

function get_msg_type(self::GraphMessageHeader_Buffer)
    return self._msg_type
end

struct GraphMessage{T}
    _header::GraphMessageHeader
    _data::T

    GraphMessage(
        header::GraphMessageHeader,
        data::T,
    ) = new(
            header,
            data,
        )
end
function serialize(self::GraphMessage, buf::Bytes, start_pos::Int64) 
    offsets = []
    pos = start_pos + sizeof(offsets)
    pos = serialize(buf, pos, self._header)
    pos = serialize(buf, pos, self._data)
    serialize(buffer, start_pos, offsets)
    return pos
end

function deserialize(buf::Bytes, pos::Ref{Int}, ::Type{T}) where { T<:GraphMessage }
    offsets = []
    pos[] += sizeof(offsets)
    GraphMessage(
        deserialize(buf, pos, GraphMessageHeader), # Deserialize header
        deserialize(buf, pos, T), # Deserialize data
    )
end
header(self::GraphMessage) = self._header
data(self::GraphMessage) = self._data

struct GraphMessage_Buffer{T}
    buffer::Vector{UInt8}
    start_pos::UInt64
end

# Offset Calculations
const GraphMessage_OFFSET_COUNT = 0
const GraphMessage_START_OFFSET = sizeof(Int) * GraphMessage_OFFSET_COUNT
const GraphMessage_header_OFFSET = 0
const GraphMessage_data_OFFSET = 0

function get_header(self::GraphMessage_Buffer)
    return self._header
end

function get_data(self::GraphMessage_Buffer)
    return self._data
end

