struct NodeId
    id::String
end

struct ChannelId
    id::String
end

@enum EdgeDirection In Out

struct EdgeId
    node_id::NodeId
    channel_id::ChannelId
    direction::EdgeDirection
end

struct GraphMessageId
    id::UInt64
end

struct GraphMessageHeader
    id::GraphMessageId
    ts_enqueued::Optional{Timestamp}
    ts_in::Optional{Timestamp}
    ts_out::Timestamp
    conflate_count::UInt64
    msg_type::Optional{String}
    parent_ids::Vector{GraphMessageId}
end

struct GraphMessage{T}
    header::GraphMessageHeader
    data::T
end

struct DutyCycleState
    first_msg_ts::Optional{Timestamp}
    oldest_msg_ts::Optional{Timestamp}
    conflate_count::UInt32
    duty_cycle::UInt128
    parent_ids::Vector{GraphMessageId}
end

@enum GraphMetricType CalcTime NodeTime QueueTime ConflateCount SentMsgCount ReceivedMsgCount

struct GraphMetricKey
    edge_id::EdgeId
    metric_type::GraphMetricType
end

struct GraphMetric{T}
    key::GraphMetricKey
    timestamp::Timestamp
    value::T
end