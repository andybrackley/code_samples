use crate::types::BufferT;

pub trait Serializer {
    fn serialize(&self, buffer: &mut BufferT, pos: usize) -> usize;
}

impl Serializer for i64 {
    fn serialize(&self, buffer: &mut BufferT, pos: usize) -> usize {
        return serialize_scalar(self, buffer, pos);
    }
}

unsafe fn write_bytes(bytes: &[u8], size: usize, buffer: &mut BufferT, pos: usize) {
    let ptr = buffer.as_mut_ptr().add(pos);
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, size);
}

// TODO:  Hoping I can do this with the template args instead...
fn serialize<T>(bytes: &[u8], buffer: &mut BufferT, pos: usize) -> usize {
    let size = size_of::<T>();
    let new_pos = pos + size;

    unsafe {
        write_bytes(&bytes, size, buffer, pos);
        buffer.set_len(new_pos);
    }

    return new_pos;
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts(p as *const T as *const u8, ::core::mem::size_of::<T>())
}

pub fn serialize_scalar<T>(scalar: &T, buffer: &mut BufferT, pos: usize) -> usize {
    unsafe {
        let bytes = any_as_u8_slice(scalar);
        return serialize::<T>(bytes, buffer, pos);
    }
}

pub fn serialize_option<T: Serializer>(v: &Option<T>, buffer: &mut BufferT, pos: usize) -> usize {
    let mut pos = pos;

    if let Some(v) = v {
        pos = serialize_scalar(&1, buffer, pos);
        pos = v.serialize(buffer, pos);
    } else {
        pos = serialize_scalar(&0, buffer, pos);
    }

    return pos;
}

pub fn serialize_vec<T: Serializer>(vec: &Vec<T>, buffer: &mut BufferT, pos: usize) -> usize {
    let len = vec.len();
    let mut new_pos = serialize_scalar(&len, buffer, pos);

    // TODO: Can I just create flatbuffer here and simply flatten
    //       all items into memory
    for item in vec {
        new_pos = item.serialize(buffer, new_pos);
    }

    return new_pos;
}
