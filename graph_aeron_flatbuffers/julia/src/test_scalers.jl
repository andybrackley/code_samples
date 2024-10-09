include("messages/book_update.jl")
include("../serialize_raw/src/generated/book_update.jl")

function test(items:: Vector{T}) where {T}
    typename = string(T)
    scalarStream = IOBuffer()

    serializeTime = @timed serialize(scalarStream, items)
    println("SerializeTime::$typename == $serializeTime")

    filename = "..\\serialized\\adhoc\\julia.scalars.$typename.bin"
    open(filename, "w") do file 
        write(file, take!(scalarStream))
    end
            
    readBuffer = IOBuffer()
    open(filename, "r") do file
        write(readBuffer, read(file))
    end
            
    bytes = take!(readBuffer)
    off::Int32 = 0

    new_vec_pos = @timed (deserialize(bytes, off, Vector{T}))
    println("DeserializeTime::$typename == $new_vec_pos")
    new_vec, pos = new_vec_pos.value

    isSame = items == new_vec
    println("Read Vec{$typename}::isSame::$isSame::$new_vec")
    @assert isSame
end

test(['a','b','c','d'] )
test([ Int8(10), Int8(100), Int8(120), Int8(127),])
test([ Int32(10), Int32(100), Int32(1000), Int32(999999),])
test([ Int64(10), Int64(100), Int64(1000), Int64(999999),])
test(["Test1","Test2","Test3","Test4","Test5"])

optVec::Vector{Optional{Int64}} = [
    0,   # NOTE: This is failing due to the hack of inserting a Char(0) to represent None
    100,
    nothing, 
    200,
    nothing,
    999999,
    nothing,
    12345567890
]
test(optVec) 

optStrVec::Vector{Optional{String}} = [
    "Test1",
    nothing,
    "Test2",
    nothing,
    "Test3"
]

test(optStrVec)
