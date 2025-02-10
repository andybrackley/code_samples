include("../src/Framework.jl")

using ..Framework

const Optional{T} = Union{T,Nothing}
opt1::Optional{Int64} = nothing
opt2::Optional{Int64} = 55

opt_buf = BufferDirect.Instance(128)

println("Testing of Options")
# @assert pos1 == pos2 "pos1: $pos1 != pos2: $pos2 - In the case of Optional{nothing} it should be padded to pos1 "

read_pos1 = Ref(1)
pos1 = BufferDirect.write!(opt_buf, 1, opt1, Optional{Int64})
v1 = BufferDirect.read(opt_buf, read_pos1, Optional{Int64})

read_pos2 = Ref(1)
pos2 = BufferDirect.write!(opt_buf, 1, opt2, Optional{Int64})
v2 = BufferDirect.read(opt_buf, read_pos2, Optional{Int64})
# @assert read_pos1[] == read_pos2[] "read_pos1: $read_pos1 != read_pos2: $read_pos2 - In the case of Optional{nothing} it should be padded to read_pos1 "

@assert v1 == opt1 "v1: $v1 != opt1: $opt1"
@assert v2 == opt2 "v2: $v2 != opt2: $opt2"
