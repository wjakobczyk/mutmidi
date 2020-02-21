use alloc::vec::Vec;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

///Thread-safe queue
pub struct QueueThreadSafe<T> {
    items: Mutex<RefCell<Vec<T>>>,
}

impl<T> QueueThreadSafe<T> {
    pub fn new(capacity: usize) -> Self {
        QueueThreadSafe {
            items: Mutex::new(RefCell::new(Vec::with_capacity(capacity))),
        }
    }

    pub fn enque(&self, item: T) {
        cortex_m::interrupt::free(|cs| self.items.borrow(cs).borrow_mut().push(item));
    }

    pub fn deque_all(&self, items: &mut Vec<T>) {
        cortex_m::interrupt::free(|cs| items.append(&mut self.items.borrow(cs).borrow_mut()));
    }
}
