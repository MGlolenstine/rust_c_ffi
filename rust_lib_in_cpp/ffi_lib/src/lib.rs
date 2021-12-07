use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref COUNTER: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
}

#[no_mangle]
pub extern fn increase_by_one() -> u32 {
    let mut counter = COUNTER.lock().unwrap();
    *counter += 1;
    *counter
}
