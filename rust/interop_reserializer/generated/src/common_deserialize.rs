use crate::types::BufferT;

pub trait Deserializer {
    fn deserialize<'a>(buffer: &'a BufferT, pos: &mut usize) -> &'a Self;
}

impl Deserializer for i64 {
    fn deserialize<'a>(buffer: &'a BufferT, pos: &mut usize) -> &'a Self {
        return deserialize_scalar::<i64>(buffer, pos);
    }
}

pub fn deserialize_scalar<'a, T>(buffer: &'a BufferT, pos: &mut usize) -> &'a T {
    let size = size_of::<T>();
    let value = unsafe { &*(buffer.as_ptr().add(*pos) as *const T) };

    *pos += size;
    return value;
}

pub fn deserialize_option<'a, T: Deserializer>(
    buffer: &'a BufferT,
    pos: &mut usize
) -> Option<&'a T> {
    let option_choice = *deserialize_scalar::<i32>(buffer, pos);
    if option_choice == 0 {
        return None;
    }

    let v = T::deserialize(buffer, pos);
    return Some(v);
}

// TODO: Need a much better implementation than this
//       We don't really want to create a new Vector and
//       deserialize the items into it
pub fn deserialize_vec<'a, T: Deserializer>(buffer: &'a BufferT, pos: &mut usize) -> Vec<&'a T> {
    let len = deserialize_scalar::<usize>(buffer, pos);
    let deref = *len;

    let mut vec = Vec::new();
    vec.reserve(deref);

    for _ in 0..deref {
        let t = T::deserialize(buffer, pos);
        vec.push(t);
    }

    return vec;
}
