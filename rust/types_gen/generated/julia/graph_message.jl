mutable struct GraphMessageId
    _id::UInt64

    function GraphMessageId(
        id::UInt64,
    ) 
        return new(
            id,
        )
    end
    
    function get_id(self::GraphMessageId)
        return self._id
    end
     
    function set_id(self::GraphMessageId, value::UInt64)
        return self._id = value
    end
end

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

    function GraphMessageHeader(
        id::GraphMessageId,
        parent_ids::Array{GraphMessageId},
        ts_enqueued::Optional{Timestamp},
        ts_in::Optional{Timestamp},
        conflate_count::UInt64,
        msg_type::Optional{String},
    ) 
        return new(
            id,
            parent_ids,
            ts_enqueued,
            ts_in,
            conflate_count,
            msg_type,
        )
    end
    
    function get_id(self::GraphMessageHeader)
        return self._id
    end
    
    function get_parent_ids(self::GraphMessageHeader)
        return self._parent_ids
    end
    
    function get_ts_enqueued(self::GraphMessageHeader)
        return self._ts_enqueued
    end
    
    function get_ts_in(self::GraphMessageHeader)
        return self._ts_in
    end
    
    function get_conflate_count(self::GraphMessageHeader)
        return self._conflate_count
    end
    
    function get_msg_type(self::GraphMessageHeader)
        return self._msg_type
    end
    
end

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

    function GraphMessage(
        header::GraphMessageHeader,
        data::T,
    ) 
        return new(
            header,
            data,
        )
    end
    
    function get_header(self::GraphMessage)
        return self._header
    end
    
    function get_data(self::GraphMessage)
        return self._data
    end
    
end

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

