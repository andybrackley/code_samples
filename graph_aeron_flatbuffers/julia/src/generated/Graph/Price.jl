# automatically generated by the FlatBuffers compiler, do not modify

Graph.eval(quote


FlatBuffers.@STRUCT struct Price
    value::Float64
end
FlatBuffers.@ALIGN(Price, 8)

Price(buf::AbstractVector{UInt8}) = FlatBuffers.read(Price, buf)
Price(io::IO) = FlatBuffers.deserialize(io, Price)

end)
