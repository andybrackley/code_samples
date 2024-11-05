use std::ptr;

use bytemuck::{ bytes_of, NoUninit };

use crate::types::{ calc_padding, Constants, OptionIdType };

// This seems to cause issue if I move a type to another module.
// unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
//     ::core::slice::from_raw_parts(p as *const T as *const u8, ::core::mem::size_of::<T>())
// }

pub fn serialize_scalar<T: NoUninit>(v: &T, buffer: &mut [u8], pos: usize) -> usize {
    let ptr = unsafe { buffer.as_mut_ptr().add(pos) };
    let bytes = bytes_of(v).as_ptr();
    let type_size = ::core::mem::size_of::<T>();

    unsafe {
        std::ptr::copy_nonoverlapping(bytes, ptr, type_size);
    }

    let new_pos = pos + type_size;
    return new_pos;
}

pub fn serialize_option<T: NoUninit>(v: &Option<T>, buffer: &mut [u8], pos: usize) -> usize {
    let is_some: OptionIdType = if v.is_some() {
        Constants::OPTION_SOME
    } else {
        Constants::OPTION_NONE
    };
    let mut pos = serialize_scalar(&is_some, buffer, pos);

    if is_some == 1 {
        pos = serialize_scalar(&v.unwrap(), buffer, pos);
    } else {
        pos += size_of::<T>();
    }
    return pos;
}

pub fn serialize_vec<T: NoUninit>(vec: &Vec<T>, buffer: &mut [u8], pos: usize) -> usize {
    let vec_len = vec.len();
    let mut new_pos = serialize_scalar::<usize>(&vec_len, buffer, pos);

    new_pos += calc_padding(new_pos);
    for i in 0..vec.len() {
        new_pos = serialize_scalar(&vec[i], buffer, new_pos);
    }
    return new_pos;
}
