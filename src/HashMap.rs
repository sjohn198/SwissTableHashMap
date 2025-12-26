use std::mem::MaybeUninit;
use std::ptr;

struct HashMap <K, V> {
    control: Vec<u8>,
    entries: *mut MaybeUninit<(K, V)>,
    capacity: usize,
    num_items: usize
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            control: Vec::new(),
            entries: ptr::null_mut(),
            capacity: 0,
            num_items: 0
        }
    }
}