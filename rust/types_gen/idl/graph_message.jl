include("common.jl")

struct GraphMessageId
    id::UInt64
end

struct GraphMessageHeader
    id::GraphMessageId
    parent_ids::Array{GraphMessageId}
    ts_enqueued::Optional{Timestamp}
    ts_in::Optional{Timestamp}
    conflate_count::UInt64
    msg_type::Optional{String}
end

struct GraphMessage{T}
    header::GraphMessageHeader
    data::T
end

