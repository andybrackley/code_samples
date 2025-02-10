include("../src/Framework.jl")

using ..Framework

opt_buf = BufferDirect.Instance(100)
pos = BufferDirect.write!(opt_buf, 1, "xxxx", Optional{String})
read_opt_str = BufferDirect.read(opt_buf, Ref(1), Optional{String})
@assert read_opt_str == "xxxx"

arr = Array{UInt8,1}(undef, 4)
sizeof(arr)

arr[1] = 5

opt_testin::Optional{Testing.TestInner} = ti
opt_testin_buf::Optional{Testing.TestInner_Buffer} = tbi
opt_testbuf_non::Optional{Testing.TestInner} = nothing

sizeof(opt_testin)
sizeof(opt_testin_buf)
sizeof(opt_testbuf_non)


char_buf = BufferDirect.Instance(10)
null_char = '\0'
BufferDirect.write!(char_buf, 1, null_char)

println(char_buf)

str_buf = BufferDirect.Instance(256)
str = "This is a test str"
BufferDirect.write!(str_buf, 1, str, String)

read_str = BufferDirect.read(str_buf, Ref(1), String)
@assert str == read_str "Strings differ: '$str' != '$read_str"


vec_str = ["Str1", "Str2", "Str3"]
BufferDirect.write!(str_buf, 1, vec_str, Vector{String})

read_vec_str = BufferDirect.read(str_buf, Ref(1), Vector{String})
@assert vec_str == read_vec_str

