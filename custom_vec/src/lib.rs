use std::ptr;

pub struct SimpleVec<T> {
    data: *mut T,
    length: usize,
    capacity: usize,
}

impl<T> SimpleVec<T> {
    pub fn new() -> Self {
        Self {
            data: ptr::null_mut(),
            length: 0,
            capacity: 0,
        }
    }

    pub fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            1
        } else {
            2 * self.capacity
        };

        let new_data = {
            let old_layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
            let new_layout = std::alloc::Layout::array::<T>(new_capacity).unwrap();
            let new_data = unsafe { std::alloc::alloc(new_layout) as *mut T };
            if !self.data.is_null() {
                unsafe {
                    ptr::copy_nonoverlapping(self.data, new_data, self.length);
                    std::alloc::dealloc(self.data as *mut u8, old_layout);
                }
            }
            new_data
        };

        self.capacity = new_capacity;
        self.data = new_data;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        match index < self.length {
            true => Some(unsafe { &*(self.data.add(index)) }),
            false => None,
        }
    }

    pub fn push(&mut self, val: T) {
        if self.length == self.capacity {
            self.grow();
        }

        unsafe {
            ptr::write(self.data.add(self.length), val);
        }
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }

        self.length -= 1;
        let val = unsafe { ptr::read(self.data.add(self.length)) };
        Some(val)
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleVec;

    #[test]
    fn test_new() {
        let vec: SimpleVec<i32> = SimpleVec::new();
        assert_eq!(vec.get(0), None);
    }

    #[test]
    fn test_push_and_get() {
        let mut vec = SimpleVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        assert_eq!(*vec.get(0).unwrap(), 1);
        assert_eq!(*vec.get(1).unwrap(), 2);
        assert_eq!(*vec.get(2).unwrap(), 3);
        assert_eq!(vec.get(3), None);
    }

    #[test]
    fn test_pop() {
        let mut vec = SimpleVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert_eq!(vec.pop(), Some(3));
        assert_eq!(vec.pop(), Some(2));
        assert_eq!(vec.pop(), Some(1));
        assert_eq!(vec.pop(), None);
    }

    #[test]
    fn test_push_pop() {
        let mut vec = SimpleVec::new();
        vec.push(10);
        vec.push(20);
        vec.push(30);
        assert_eq!(vec.pop(), Some(30));
        vec.push(40);
        assert_eq!(*vec.get(1).unwrap(), 20);
        assert_eq!(*vec.get(2).unwrap(), 40);
        assert_eq!(vec.get(3), None);
    }

    #[test]
    fn test_growth() {
        let mut vec = SimpleVec::new();
        for i in 0..100 {
            vec.push(i);
        }

        for i in (0..100).rev() {
            assert_eq!(vec.pop(), Some(i));
        }
        assert_eq!(vec.pop(), None);
    }
}
