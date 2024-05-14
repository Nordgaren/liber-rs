use std::sync::OnceLock;

#[link(name = "kernel32", kind = "raw-dylib")]
extern "system" {
    fn GetModuleHandleA(name: *const u8) -> usize;
}

pub fn get_base_address() -> usize {
    static BASE_ADDRESS: OnceLock<usize> = OnceLock::new();
    *BASE_ADDRESS.get_or_init(|| unsafe { GetModuleHandleA(std::ptr::null::<u8>()) })
}
