use crate::types::BufferT;

pub fn deserialize_scalar<T>(buffer: &mut BufferT, pos: &mut usize) -> Option<T> {
    *pos += size_of::<T>();
    return None;
}

pub fn deserialize_option<T>(v: &Option<T>, buffer: &mut BufferT, pos: &mut usize) -> Option<T> {
    let mut pos = pos;
    let option_choice: i64 = deserialize_scalar(buffer, pos).unwrap();

    return None;
}

pub fn deserialize_vec<T>(buffer: &mut BufferT, pos: &mut usize) -> Vec<T> {
    let len: i64 = deserialize_scalar(buffer, pos).unwrap();
    // TODO: Need to figure out how to implement this
    return Vec::new();
}
