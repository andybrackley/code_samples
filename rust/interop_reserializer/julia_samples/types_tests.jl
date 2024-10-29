@enum Fruit banana = 1 apple = 2 orange = 3

@enum Status begin
    Live = 1
end

mutable struct BookUpdate
    time::Int64
    timestamp_exch::Union{Int64,Nothing}
    instId::Int64
    updateType::Int64

    bids::Vector{Int64}
    asks::Vector{Int64}
end
