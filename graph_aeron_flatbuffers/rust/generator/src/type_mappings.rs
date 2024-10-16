/*

Mappings:

Julia   Rust    C++

Bool	bool	bool
Char	char	char
UInt8	u8	uint8_t
UInt16	u16	uint16_t
UInt32	u32	uint32_t
UInt64	u64	uint64_t
Int8	i8	int8_t
Int16	i16	int16_t
Int32	i32	int32_t
Int64	i64	int64_t
Float32	f32	float
Float64	f64	double
String	String	std::string
Vector{T}	Vec<T>	std::vector<T>
Union{T, Nothing}	Option<T>	std::optional<T>

*/


/* 
   Julia Examples:

Common.jl

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

# Main Types:

BookUpdate.jl

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

*/

mod generator {

// Need regex = "1.5.4" in Cargo.toml
use regex::Regex;

#[derive(Debug)]
struct EnumInfo {
    name: String,
    type_name: String,
    variants: Vec<(String, u16)>,
}

#[derive(Debug)]
struct StructInfo {
    name: String,
    fields: Vec<(String, String)>,
}

// struct Field {
//     name: String,
//     prop_type: String,  // Needs more than this to support things like Union{Nothing, T} 
// }

// struct StructType {
//     name: String,
//     generics: Vector<String>,
//     fields: Vector<String>
// }

enum Languages {
    Julia,
    Rust,
    Cpp
}


fn get_type_mapping(julia_type: &str, language: &str) -> &str {
    // let mut knownTypes = HashMap<String, StructType>::new();

    ""
}

fn parse_enum_values(enum_str: &str)  {
    let enum_regex = Regex::new(r"@enum\s+(\w+)::(\w+)\s+begin\s+((?:\s*\w+\s*=\s*\d+\s*,?\s*)*)\s*end").unwrap();
    let variant_regex = Regex::new(r"(\w+)\s*=\s*(\d+)").unwrap();

    let caps = enum_regex.captures(enum_str).unwrap();
    let name = caps[1].to_string();
    let type_name = caps[2].to_string();
    let variants_str = &caps[3];

    let mut variants = Vec::new();
    for cap in variant_regex.captures_iter(variants_str) {
        let variant_name = cap[1].to_string();
        let variant_value: u16 = cap[2].parse().unwrap();
        variants.push((variant_name, variant_value));
    }

    EnumInfo {
        name,
        type_name,
    }
}

fn parse_struct(struct_str: &str) -> StructInfo {
    let struct_regex = Regex::new(r"struct\s+(\w+)\s*<:?\s*\w*\s*{\s*((?:\s*\w+\s*::\s*\w+\s*,?\s*)*)\s*}").unwrap();
    let field_regex = Regex::new(r"(\w+)\s*::\s*(\w+)").unwrap();

    let caps = struct_regex.captures(struct_str).unwrap();
    let name = caps[1].to_string();
    let fields_str = &caps[2];

    let mut fields = Vec::new();
    for cap in field_regex.captures_iter(fields_str) {
        let field_name = cap[1].to_string();
        let field_type = cap[2].to_string();
        fields.push((field_name, field_type));
    }

    StructInfo {
        name,
        fields,
    }
}

fn parse_field() {

}

}