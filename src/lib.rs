use std::alloc::{alloc, dealloc, realloc, Layout};
use std::mem::{align_of, size_of};
use std::ptr::{drop_in_place, NonNull};
use std::slice::from_raw_parts_mut;

pub struct MyVec<T> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        assert_ne!(size_of::<T>(), 0, "No zeros!");

        if self.capacity == 0 {
            let layout = Layout::array::<T>(4).expect("Could not allocate: Zero.");

            // safe: layout is 4 * size_of(T) and size_of(T) > 0.
            let ptr = unsafe { alloc(layout) } as *mut T;
            let ptr = NonNull::new(ptr).expect("Could not allocate: Null.");

            // safe: pointer is non-null and space is allocated for 4 items.
            unsafe {
                ptr.as_ptr().write(item);
            }
            self.ptr = ptr;
            self.capacity = 4;
            self.len = 1;
        } else if self.len < self.capacity {
            assert!(
                self.len * size_of::<T>() < isize::MAX as usize,
                "isize overflow"
            );

            // offset by vec.len() * size_of(T) and write to memory
            unsafe {
                self.ptr.as_ptr().add(self.len).write(item);
            }
            self.len += 1;
        } else {
            let new_capacity = self.capacity.checked_mul(2).expect("Capacity overflow");
            let size = size_of::<T>() * self.capacity;
            let align = align_of::<T>();
            size.checked_add(size % align)
                .expect("Layout from size and align failed.");

            let new_size = size_of::<T>() * new_capacity;

            let ptr = unsafe {
                let layout = Layout::from_size_align_unchecked(size, align);
                let ptr = realloc(self.ptr.as_ptr() as *mut u8, layout, new_size);
                let ptr = NonNull::new(ptr as *mut T).expect("Could not reallocate.");
                ptr.as_ptr().add(self.len).write(item);
                ptr
            };

            self.ptr = ptr;
            self.capacity = new_capacity;
            self.len += 1;
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        Some(unsafe { &*self.ptr.as_ptr().add(index) })
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        unsafe {
            drop_in_place(from_raw_parts_mut(self.ptr.as_ptr(), self.len));
            let layout =
                Layout::from_size_align_unchecked(size_of::<T>() * self.capacity, align_of::<T>());
            dealloc(self.ptr.as_ptr() as *mut u8, layout)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec: MyVec<usize> = MyVec::new();
        vec.push(1usize);
        vec.push(2);
        vec.push(3);
        vec.push(3);
        vec.push(3);

        assert_eq!(vec.capacity(), 8);
        assert_eq!(vec.len(), 5);
    }
}
