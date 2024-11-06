use std::ptr::NonNull;

#[derive(Debug)]
pub struct RawArray<T> {
    count: usize,
    ptr: NonNull<T>,
}
impl<T> RawArray<T> {
    pub fn from_vec(mut v: Vec<T>) -> RawArray<T> {
        let size = v.len();
        let ptr = NonNull::new(v.as_mut_ptr()).expect("Vector pointer should be non null");
        std::mem::forget(v);

        RawArray {
            count: size,
            ptr,
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.count {
            unsafe { Some(&*self.ptr.as_ptr().add(index)) }
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.count {
            unsafe { Some(&mut *self.ptr.as_ptr().add(index)) }
        } else {
            None
        }
    }

    pub fn to_vec(&self) -> Vec<&T> {
        let mut vec: Vec<&T> = Vec::with_capacity(self.count);
        unsafe {
            for i in 0..self.count {
                let ptr = &*self.ptr.as_ptr().add(i);
                vec.push(ptr);
            }
        }

        vec
    }
}
