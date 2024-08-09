# bufferbuffer

This is an implementation of the **Double Buffer** design pattern from 'Game Programming Patterns' by Robert Nystrom.

In a simulation, you often have to do a lot of processing to prepare the next "frame", but if you're
iterating through the current-state data while mutating it, things can slip.  The Double Buffer design
pattern solves this by keepint two copies of the simuilation state (or any variable): the "current"
(or previous) state which is immutable, and the "next" (or future) state which is being prepared.  When
a turn of the simulation is completed, you simply switch the buffers.

Unlike other implementations on crates.io, this one wraps both buffers in `std::cell::RefCell` so that
it is possible to borrow one buffer as mutable at the same time the other is borrows as immutable --
a typical use case is to iterate over objects in the world (current state) and write updated versions of 
them to the next state.

## Usage

For a simple variable:

    let mut my_double_buf: DoubleBuffer<i32> = DoubleBuffer::new(0,0);
    *my_double_buf.next() += 10;
    my_double_buf.switch();
    assert_eq!(*my_double_buf.current(), 10);

For a vector:

    let mut my_double_buf: DoubleBuffer<Vec<i32>> = DoubleBuffer::new( vec!(2,4,6), Vec::new());
    for number in my_double_buf.current().iter() {
        my_double_buf.next().push(*number + 1);
    }
    my_double_buf.switch();
    assert_eq!(*my_double_buf.current(), vec!(3,5,7));

