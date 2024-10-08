include("common.jl")

export BookUpdate, BookUpdateFull

mutable struct BookUpdate
    time:: Timestamp
    timestamp_exch::Optional{Timestamp}
    instId:: InstrumentId
    updateType:: BookUpdateType

    bids::Vector{Level}
    asks::Vector{Level}
end


mutable struct BookUpdateFull 
    time:: Timestamp
    timestamp_exch::Optional{Timestamp}
    instId:: InstrumentId

    bids::Vector{Level}
    asks::Vector{Level}
end

