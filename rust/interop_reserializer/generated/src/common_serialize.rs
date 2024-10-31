use crate::types::BufferT;

unsafe fn write_bytes(bytes: &[u8], pos: usize, size: usize, buffer: &mut BufferT) {
    let ptr = buffer.as_mut_ptr().add(pos);
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, size);
}

// TODO:  Hoping I can do this with the template args instead...
fn serialize<T>(bytes: &[u8], buffer: &mut BufferT, pos: usize) -> usize {
    let size = size_of::<T>();
    let new_pos = pos + size;

    unsafe {
        write_bytes(&bytes, pos, size, buffer);
        buffer.set_len(new_pos);
    }

    return new_pos;
}

pub fn serialize_i32(scalar: &i32, buffer: &mut BufferT, pos: usize) -> usize {
    serialize::<i32>(&scalar.to_ne_bytes(), buffer, pos)
}

pub fn serialize_i64(scalar: &i64, buffer: &mut BufferT, pos: usize) -> usize {
    serialize::<i64>(&scalar.to_ne_bytes(), buffer, pos)
}

pub fn serialize_usize(scalar: &usize, buffer: &mut BufferT, pos: usize) -> usize {
    serialize::<usize>(&scalar.to_ne_bytes(), buffer, pos)
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

// pub fn serialize_scalar<T>(scalar: &T, buffer: &mut BufferT, pos: usize) -> usize {
//     let size = size_of::<T>();

//     unsafe {
//         let bytes = any_as_u8_slice(scalar);
//         let ptr = buffer.as_mut_ptr(); //.add(buffer.len());
//         std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, size);

//         let new_pos = pos + size;
//         buffer.set_len(new_pos);
//         return new_pos;
//     }
// }

pub fn serialize_option<T>(v: &Option<T>, buffer: &mut BufferT, pos: usize) -> usize {
    let mut pos = pos;

    if let Some(v) = v {
        pos = serialize_i32(&1, buffer, pos);

        // TODO Need to serialize the actual value
    } else {
        pos = serialize_i32(&0, buffer, pos);
    }

    return pos;
}

pub fn serialize_vec<T>(scalar: &Vec<T>, buffer: &mut BufferT, pos: usize) -> usize {
    let len = scalar.len();
    let new_pos = serialize_usize(&len, buffer, pos);

    // TODO: Need to figure out how to implement this
    return new_pos;
}
