use std::borrow::BorrowMut;

use bytemuck::{ checked::{ try_from_bytes, CheckedCastError }, AnyBitPattern };

use crate::types::BufferT;

pub fn deserialize_scalar<'a, T>(buffer: &'a [u8], pos: &mut usize) -> &'a T {
    let size = size_of::<T>();
    let value = unsafe { &*(buffer.as_ptr().add(*pos) as *const T) };

    *pos += size;
    return value;
}

pub fn deserialize_vec<'a, T: AnyBitPattern>(buffer: &'a [u8], offset: &mut usize) -> &'a [T] {
    let vec_size = deserialize_scalar::<usize>(&buffer, offset.borrow_mut());
    let end = *offset + size_of::<T>() * vec_size;

    let slice = &buffer[*offset..end];
    let x: &[T] = bytemuck::try_cast_slice(slice).unwrap();
    *offset = end;
    return x;
}

// TODO: I think I want to return a reference here
pub fn deserialize_option<'a, T: AnyBitPattern + Copy>(
    buffer: &'a [u8],
    offset: &mut usize
) -> Option<T> {
    let is_some = deserialize_scalar::<i64>(buffer, offset);
    if *is_some == 0 {
        return None;
    }

    let v = deserialize_scalar::<T>(buffer, offset);
    return Some(*v);
}
