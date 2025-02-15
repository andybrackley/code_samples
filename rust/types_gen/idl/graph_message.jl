include("common.jl")

mutable struct GraphMessageId
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

struct GraphMessageString
    header::GraphMessageHeader
    data::String
end
