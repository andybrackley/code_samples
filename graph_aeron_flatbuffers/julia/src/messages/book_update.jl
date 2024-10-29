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

abstract type Test{T} end


struct TestImpl{T} <: Test{T}

end

struct TestImpl2{Int64} <: Test{Int64}

end

const TestAlias{T} = TestImpl2{T}
