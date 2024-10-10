include("messages/book_update.jl")
include("../serialize_raw/src/generated/book_update.jl")

module JuliaSerialization

const BookUpdate = Main.BookUpdate
using Serialization

function run_julia_serialize_test(update::BookUpdate, stream::IOBuffer)
    buf = serialize(stream, update)
    return stream
end

function run_julia_deserialize_test(stream::IOBuffer)
    update = deserialize(stream)
end

end


function run_serialize_test(update::BookUpdate, stream::IOBuffer) 
    write(stream, Int32(0))
    size = serializeBookUpdate(stream, t)
    seekstart(stream)
    write(stream, Int32(size))
    return stream    
end

function run_deserialize_test(bytes::Bytes)
    return deserializeBookUpdate(bytes)
end



bids:: Vector{Level} = [ Level(1), Level(2), Level(3) ]
asks:: Vector{Level} = [ Level(4), Level(3), Level(2) ]

# t = BookUpdate(Timestamp(100), nothing, InstrumentId(ExchangeDeribit, "InstId::1234"), BookUpdateTypeSnapshot, bids, asks)

t = BookUpdate(Timestamp(100), InstrumentId(ExchangeDeribit, "InstId::1234"), BookUpdateTypeSnapshot, bids, asks)

function warmup_julia_serialization_test() 
    for i in 1:5
        stream = IOBuffer()
        s = JuliaSerialization.run_julia_serialize_test(t, stream)
        seekstart(stream)
        JuliaSerialization.run_julia_deserialize_test(s)
    end
end

function run_julia_serialization_test()
    stream = IOBuffer()
    timed_julia_serialization = @timed JuliaSerialization.run_julia_serialize_test(t, stream)
    
    seekstart(stream)
    timed_julia_deserialization = @timed JuliaSerialization.run_julia_deserialize_test(stream)
    
    println("timed::run_julia::serialize_test::$timed_julia_serialization")
    println("timed::run_julia::deserialize_test::$timed_julia_deserialization")
end

function warmup_serialization_test() 
    for i in 1:5 
        stream = IOBuffer()
        run_serialize_test(t, stream)
        seekstart(stream)
        bytes = take!(stream)
        run_deserialize_test(bytes)
    end
end

function run_serialization_test()
    stream = IOBuffer()
    stream_timed = @timed run_serialize_test(t, stream)
    println("timed::run_serialize_test::$stream_timed")
    
    filename = "..\\serialized\\adhoc\\julia.bookupdate.bin"
    
    open(filename, "w") do file 
        write(file, take!(stream))
    end
    
    println("Written $size bytes to stream")
    
    readBuffer = IOBuffer()
    open(filename, "r") do file
        write(readBuffer, read(file))
    end
        
    bytes = take!(readBuffer)
    
    timed_deserialize = @timed run_deserialize_test(bytes)
    println("timed::run_deserialize_test::$stream_timed")
    
    value = timed_deserialize.value
    println("Result: $value")
end


println("============== Start Running Julia Serialization Tests ===================")
warmup_julia_serialization_test()
run_julia_serialization_test()
println("============== End Running Julia Serialization Tests ===================")

println("============== Start Running Adhoc Serialization Tests ===================")
warmup_serialization_test()
run_serialization_test()
println("============== End Running Adhoc Serialization Tests ===================")
