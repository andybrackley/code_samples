use std::mem;
use std::ptr;

#[cfg(test)]
struct Example {
    buffer: Vec<u8>,
}

impl Example {
    fn new(a: i8, b: &[i32]) -> Self {
        let a_size = mem::size_of::<i8>();
        let b_size = b.len() * mem::size_of::<i32>();

        let mut buffer = vec![0u8; a_size + b_size];

        let a_bytes = a.to_ne_bytes();
        buffer[..a_size].copy_from_slice(&a_bytes);

        let b_offset = a_size;
        let b_bytes = unsafe { std::slice::from_raw_parts(b.as_ptr() as *const u8, b_size) };
        buffer[b_offset..b_offset + b_size].copy_from_slice(b_bytes);

        Self { buffer }
    }

    fn a(&self) -> i8 {
        let a_ptr = self.buffer.as_ptr() as *const i8;
        unsafe { ptr::read_unaligned(a_ptr) }
    }

    fn set_a(&mut self, value: i8) {
        let a_ptr = self.buffer.as_mut_ptr() as *mut i8;
        unsafe { ptr::write_unaligned(a_ptr, value) }
    }

    fn b(&self) -> &[i32] {
        let a_size = mem::size_of::<i32>();
        let b_ptr = unsafe { self.buffer.as_ptr().add(a_size) as *const i32 };
        let b_len = (self.buffer.len() - a_size) / mem::size_of::<i32>();
        unsafe { std::slice::from_raw_parts(b_ptr, b_len) }
    }

    fn set_b(&mut self, index: usize, value: i32) {
        let a_size = mem::size_of::<i32>();
        let b_ptr = unsafe { self.buffer.as_mut_ptr().add(a_size) as *mut i32 };
        unsafe { ptr::write(b_ptr.add(index), value) }
    }
}

#[test]
pub fn test() {
    let a: i8 = 42;
    let b = vec![1, 2, 3, 4];

    let mut example = Example::new(a, &b);

    println!("{}, {:?}", example.a(), example.b());
}
