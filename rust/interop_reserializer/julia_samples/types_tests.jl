@enum Int32::Status begin
   Live = 1
end

mutable struct BookUpdate
    time:: Int64
    timestamp_exch::Union{Int64, Nothing}
    instId:: Int64
    updateType:: Int64

    bids::Vector{Int64}
    asks::Vector{Int64}
end
