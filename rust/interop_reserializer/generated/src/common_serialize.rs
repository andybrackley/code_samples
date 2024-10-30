use crate::types::BufferT;

pub fn serialize_scalar<T>(scalar: &T, buffer: &mut BufferT, pos: usize) -> usize {
    return pos + size_of::<T>();
}

pub fn serialize_option<T>(v: &Option<T>, buffer: &mut BufferT, pos: usize) -> usize {
    let mut pos = pos;

    if let Some(v) = v {
        pos = serialize_scalar(&1, buffer, pos);

        // TODO Need to serialize the actual value
    } else {
        pos = serialize_scalar(&0, buffer, pos);
    }

    return pos;
}

pub fn serialize_vec<T>(scalar: &Vec<T>, buffer: &mut BufferT, pos: usize) -> usize {
    let len = scalar.len();

    let new_pos = serialize_scalar(&len, buffer, pos);

    // TODO: Need to figure out how to implement this
    return new_pos;
}
