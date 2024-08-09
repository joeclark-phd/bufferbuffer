use std::cell::{Ref, RefCell, RefMut};


pub struct DoubleBuffer<T> {
    first: RefCell<T>,
    second: RefCell<T>,
    switched: bool,
}

/// An implementation of the Double Buffer pattern from 'Game Programming Patterns' by Robert Nystrom.
/// 
/// In a simulation, you often have to do a lot of processing to prepare the next "frame", but if you're
/// iterating through the current-state data while mutating it, things can slip.  The Double Buffer design
/// pattern solves this by keepint two copies of the simuilation state (or any variable): the "current"
/// (or previous) state which is immutable, and the "next" (or future) state which is being prepared.  When
/// a turn of the simulation is completed, you simply switch the buffers.
/// 
/// Unlike other implementations on crates.io, this one wraps both buffers in `std::cell::RefCell` so that
/// it is possible to borrow one buffer as mutable at the same time the other is borrows as immutable --
/// a typical use case is to iterate over objects in the current state and write updated versions of them
/// to the next state.
impl<T> DoubleBuffer<T> {

    pub fn new(current: T, next: T) -> Self {
        Self {
            first: RefCell::new(current),
            second: RefCell::new(next),
            switched: false,
        }
    }

    /// Get an immutable reference to the current-state buffer.
    pub fn current(&self) -> Ref<T> {
        match self.switched {
            false => self.first.borrow(),
            true => self.second.borrow(),
        }
    }

    /// Get a mutable reference to the next-state buffer.
    pub fn next(&self) -> RefMut<T> {
        match self.switched {
            false => self.second.borrow_mut(),
            true => self.first.borrow_mut(),
        }
    }

    /// Switch the "current" and "next" buffers.
    pub fn switch(&mut self) {
        self.switched = !self.switched;
    }

}


#[cfg(test)]
mod tests {
    use crate::DoubleBuffer;


    #[test]
    fn switching_buffers_works() {
        let mut my_double_buf: DoubleBuffer<i32> = DoubleBuffer::new(0,0);
        *my_double_buf.next() += 10;
        assert_eq!(*my_double_buf.current(), 0);
        my_double_buf.switch();
        assert_eq!(*my_double_buf.current(), 10);
        my_double_buf.switch();
        assert_eq!(*my_double_buf.current(), 0);
    }


    #[test]
    fn writing_from_current_to_next_works() {
        let mut my_double_buf: DoubleBuffer<Vec<i32>> = DoubleBuffer::new( vec!(2,4,6), Vec::new());
        for number in my_double_buf.current().iter() {
            my_double_buf.next().push(*number + 1);
        }
        my_double_buf.switch();
        assert_eq!(*my_double_buf.current(), vec!(3,5,7));
        *my_double_buf.next() = Vec::new();
        for number in my_double_buf.current().iter() {
            my_double_buf.next().push(*number + 1);
        }
        my_double_buf.switch();
        assert_eq!(*my_double_buf.current(), vec!(4,6,8));

     }


}
