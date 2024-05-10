#[repr(C)]
#[derive(Debug)]
pub struct FD4Time {
    time: f32,
}

const _: () = assert!(std::mem::size_of::<FD4Time>() == 0x4);
