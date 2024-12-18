mutable struct BookUpdate
    time::Int8
    timestamp_exch::Optional{Int32}
    instId::Int64
    updateType::Int128

    bids::Vector{Int32}
    asks::Vector{Int64}
end
