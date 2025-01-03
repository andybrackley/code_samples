struct GraphMessageId
    id::UInt64
end

struct GraphMessageHeader
    id::GraphMessageId
    parent_ids::Array
    ts_enqueued::Optional
    ts_in::Optional
    conflate_count::UInt64
    msg_type::Optional
end

struct GraphMessage
    header::GraphMessageHeader
    data::T
end

