use crate::{CppClass, DestructorFn, VTable};

#[repr(C)]
pub struct FD4TimeVTable<C: VTable> {
    destructor: DestructorFn<C>,
}
const _: () = assert!(std::mem::size_of::<FD4TimeVTable<FD4TimeType>>() == 0x8);

pub type FD4Time = CppClass<FD4TimeType>;
const _: () = assert!(std::mem::size_of::<FD4Time>() == 0x10);

#[repr(C)]
#[derive(Debug)]
pub struct FD4TimeType {
    time: f32,
}
const _: () = assert!(std::mem::size_of::<FD4TimeType>() == 0x4);

impl VTable for FD4TimeType {
    type Table = FD4TimeVTable<FD4TimeType>;
    const TABLE: &'static Self::Table = &FD4TimeVTable::new();
}

pub trait FD4TimeTrait {
    extern "C" fn destructor(&self) {}
}

impl FD4TimeTrait for FD4Time {}

impl<C: VTable> FD4TimeVTable<C>
where
    CppClass<C>: FD4TimeTrait,
{
    pub const fn new() -> Self {
        Self {
            destructor: <CppClass<C> as FD4TimeTrait>::destructor,
        }
    }
}
