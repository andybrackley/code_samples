use std::{ borrow::BorrowMut, ptr };

use bytemuck::{ checked::{ try_from_bytes, CheckedCastError }, AnyBitPattern };

use crate::types::{ calc_padding, BufferT, Constants };

pub fn deserialize_scalar<'a, T: Copy>(buffer: &'a [u8], pos: &mut usize) -> T {
    let size = size_of::<T>();
    debug_assert!(
        buffer.len() >= *pos + size,
        "insufficient capacity for emplace_scalar, needed {} got {}",
        size,
        buffer.len()
    );

    let value = unsafe { &*(buffer.as_ptr().add(*pos) as *const T) };

    // let value: T = unsafe { ptr::read_unaligned(buffer.as_ptr() as *const T) };

    *pos += size;
    return *value;
}

pub fn deserialize_vec<'a, T: AnyBitPattern>(buffer: &'a [u8], offset: &mut usize) -> &'a [T] {
    let vec_size = deserialize_scalar::<usize>(&buffer, offset.borrow_mut());

    debug_assert!(
        buffer.len() >= *offset + vec_size * size_of::<T>(),
        "insufficient capacity for emplace_scalar, needed {} got {}",
        vec_size,
        buffer.len()
    );

    *offset += calc_padding(*offset);

    // This seems to blow up when I use things other than an i64
    // with error: TargetAlignmentGreaterAndInputNotAligned
    //    https://docs.rs/bytemuck/latest/bytemuck/enum.PodCastError.html
    // let end = *offset + size_of::<T>() * vec_size;
    // let slice = &buffer[*offset..end];
    // let x: &[T] = bytemuck::try_cast_slice(slice).unwrap();

    let ptr = unsafe { buffer.as_ptr().add(*offset) as *const T };
    let x: &[T] = unsafe { std::slice::from_raw_parts(ptr, vec_size) };

    *offset += vec_size * size_of::<T>();
    return x;
}

// TODO: I think I want to return a reference here
pub fn deserialize_option<'a, T: AnyBitPattern + Copy>(
    buffer: &'a [u8],
    offset: &mut usize
) -> Option<T> {
    let is_some = deserialize_scalar::<i8>(buffer, offset);
    if is_some == Constants::OPTION_NONE {
        *offset += size_of::<T>();
        return None;
    }

    let v = deserialize_scalar::<T>(buffer, offset);
    return Some(v);
}
