struct Timestamp
    _stamp::UInt64

    function Timestamp(
        stamp::UInt64,
    ) 
        return new(
            stamp,
        )
    end
    
    function get_stamp(self::Timestamp)
        return self._stamp
    end
    
end

struct Timestamp_Buffer
    buffer::Vector{UInt8}
    start_pos::UInt64
end

# Offset Calculations
const Timestamp_OFFSET_COUNT = 0
const Timestamp_START_OFFSET = sizeof(Int) * Timestamp_OFFSET_COUNT
const Timestamp_stamp_OFFSET = 0

function get_stamp(self::Timestamp_Buffer)
    return self._stamp
end

