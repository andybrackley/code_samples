mutable struct GraphMessageId
    id::UInt64
end

struct TestHeader
    stamp::UInt64
    parent_ids::Vector{GraphMessageId}
    opt_id::Optional{GraphMessageId}
    other_ids::Vector{GraphMessageId}
end