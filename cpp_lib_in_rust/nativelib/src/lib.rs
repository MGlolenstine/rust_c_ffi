pub fn increase_counter_by_one() -> u32{
    unsafe{nativelib_sys::increase_counter_by_one()}
}