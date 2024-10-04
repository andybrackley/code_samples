
mutable struct T{A, B} 
    bids::Vector{A}
    asks::Vector{B}
end

const Ints = T{Int64, Int64}

i1 = Ints([], [])


function test(::Type{$T}, ::Type{$TT}, sym) 
    println("In Test")
end
