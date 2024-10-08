@enum BookUpdateType::UInt8 begin
    BookUpdateTypeUpdate = 0
    BookUpdateTypeSnapshot = 1
end

@enum Exchange::UInt16 begin
    ExchangeInternal = 0
    ExchangeBinance = 1
    ExchangeBitstamp = 2
    ExchangeBitfinex = 3
    ExchangeBittrex = 4
    ExchangeCoinbase = 5
    ExchangeDeribit = 6
    ExchangeGateio = 7
    ExchangeGemini = 8
    ExchangeItbit = 9
    ExchangeKraken = 10
    ExchangeLmax = 11
    ExchangeOkcoin = 12
    ExchangeOkx = 13
end

# generic types
const Optional{T} = Union{T,Nothing}

struct Timestamp 
    value::Int64
end

struct Level 
    value::Int64
end

struct InstrumentId 
    exchange:: Exchange
    id::String
end
