use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

static ATOMIC_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT; // starts at zero

pub fn incr() -> usize {
    // we add one to return the new value rather than the old value
    1 + ATOMIC_COUNTER.fetch_add(1, Ordering::SeqCst)
}
