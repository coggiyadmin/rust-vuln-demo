// Unsafe memory operations demonstrating common Rust security pitfalls

// CWE-190: integer overflow — unchecked arithmetic
pub fn calculate_buffer_size(count: u32, item_size: u32) -> usize {
    // CWE-190: can overflow if count * item_size > u32::MAX
    (count * item_size) as usize
}

// CWE-416: use-after-free via raw pointer manipulation
pub unsafe fn use_after_free_demo() {
    let mut data = vec![1u8, 2, 3, 4, 5];
    let ptr = data.as_mut_ptr();
    drop(data);                                        // data freed here
    *ptr = 42;                                         // CWE-416: write to freed memory
}

// CWE-476: null pointer dereference via unwrap on None
pub fn get_user_name(users: &std::collections::HashMap<u64, String>, id: u64) -> String {
    users.get(&id).unwrap().clone()                    // CWE-476: panics if id not found
}

// CWE-122: heap buffer overflow via unsafe slice
pub unsafe fn copy_data(src: *const u8, dst: *mut u8, len: usize) {
    // CWE-122: no bounds check — caller controls len
    std::ptr::copy_nonoverlapping(src, dst, len);
}

// CWE-401: memory leak — Box not freed (simplified demo)
pub fn allocate_without_free() -> *mut [u8; 1024] {
    let boxed = Box::new([0u8; 1024]);
    Box::into_raw(boxed)                               // CWE-401: caller must free, easy to forget
}

// CWE-680: integer overflow leading to buffer overflow
pub fn read_packet(buf: &mut Vec<u8>, header_len: u8, body_len: u8) {
    let total = header_len as usize + body_len as usize;
    if total > buf.capacity() {
        buf.reserve(total);
    }
    // If header_len + body_len wraps, reserve is called with wrong size
    unsafe {
        buf.set_len(total);                            // CWE-680: uninitialized memory exposed
    }
}

// CWE-134: format string injection via user-controlled format
pub fn log_message(format_str: &str, value: &str) {
    // In a C FFI context this would be format string injection
    // Rust prevents direct format injection but FFI calls can be vulnerable
    println!("{}", format_str.replace("{}", value));
}

// Unsafe Send implementation — CWE-362 / data race
pub struct SharedState {
    pub counter: *mut u64,
}

// CWE-362: unsafe manual Send allows data races across threads
unsafe impl Send for SharedState {}
unsafe impl Sync for SharedState {}
