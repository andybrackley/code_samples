use crate::types::BufferT;

pub fn deserialize_scalar<'a, T>(buffer: &'a BufferT, pos: &mut usize) -> &'a T {
    let size = size_of::<T>();
    let value = unsafe { &*(buffer.as_ptr().add(*pos) as *const T) };

    *pos += size;
    return value;
}

pub fn deserialize_option<T>(buffer: &BufferT, pos: &mut usize) -> Option<T> {
    // let mut pos = pos;
    let option_choice = *deserialize_scalar::<i32>(buffer, pos);

    return None;
}

pub fn deserialize_vec<T>(buffer: &BufferT, pos: &mut usize) -> Vec<T> {
    let len = deserialize_scalar::<usize>(buffer, pos);

    // TODO: Need to figure out how to implement this

    return Vec::new();
}
