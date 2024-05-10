use widestring::WideCStr;
use crate::{CppClass, VTable};

pub type ClassNameFn<C> = extern "C" fn(&CppClass<C>) -> &'static u8;
pub type ClassNameWFn<C> = extern "C" fn(&CppClass<C>) -> &'static u16;
pub type RefByteFn<C> = extern "C" fn(&CppClass<C>) -> &'static u8;
pub type UnkAlwaysFalseFn<C> = extern "C" fn(&CppClass<C>) -> bool;
pub type FreeBaseFn<C> = extern "C" fn(&&CppClass<C>, allocator: *const u8);
pub type ClassSizeFn<C> = extern "C" fn(&CppClass<C>) -> usize;
/// The concatenated C++ class that represents
pub type DLRuntimeClass = CppClass<DLRuntimeClassType>;
#[repr(C)]
pub struct DLRuntimeClassVTable<C: VTable> {
    class_name: ClassNameFn<C>,
    class_name_w: ClassNameWFn<C>,
    ref_byte1: RefByteFn<C>,
    ref_byte2: RefByteFn<C>,
    ref_byte3: RefByteFn<C>,
    ref_byte4: RefByteFn<C>,
    unk_always_false: UnkAlwaysFalseFn<C>,
    free_base: FreeBaseFn<C>,
    class_size: ClassSizeFn<C>,
}

const _: () = assert!(std::mem::size_of::<DLRuntimeClassVTable<DLRuntimeClassType>>() == 0x48);

impl<C: VTable> DLRuntimeClassVTable<C> where CppClass<C>: DLRuntimeClassTrait {
    pub const fn new() -> Self {
        Self {
            class_name: <CppClass<C> as DLRuntimeClassTrait>::class_name,
            class_name_w: <CppClass<C> as DLRuntimeClassTrait>::class_name_w,
            ref_byte1: <CppClass<C> as DLRuntimeClassTrait>::ref_byte1,
            ref_byte2: <CppClass<C> as DLRuntimeClassTrait>::ref_byte2,
            ref_byte3: <CppClass<C> as DLRuntimeClassTrait>::ref_byte3,
            ref_byte4: <CppClass<C> as DLRuntimeClassTrait>::ref_byte4,
            unk_always_false: <CppClass<C> as DLRuntimeClassTrait>::unk_always_false,
            free_base: <CppClass<C> as DLRuntimeClassTrait>::free_base,
            class_size: <CppClass<C> as DLRuntimeClassTrait>::class_size,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct DLRuntimeClassType {
    _class_name: &'static u8,
    _class_name_w: &'static u16,
}

const _: () = assert!(std::mem::size_of::<DLRuntimeClassType>() == 0x10);

impl VTable for DLRuntimeClassType {
    type Table = DLRuntimeClassVTable<DLRuntimeClassType>;
    const TABLE: &'static Self::Table = &DLRuntimeClassVTable::new();
}


impl DLRuntimeClassType {
    pub const fn new(class_name: &'static str, class_name_w: &'static WideCStr) -> DLRuntimeClassType {
        unsafe {
            Self { _class_name: &*class_name.as_ptr(), _class_name_w: &*class_name_w.as_ptr() }
        }
    }
}

pub trait DLRuntimeClassTrait {
    extern "C" fn class_name(&self) -> &'static u8;
    extern "C" fn class_name_w(&self) -> &'static u16;
    extern "C" fn ref_byte1(&self) -> &'static u8;
    extern "C" fn ref_byte2(&self) -> &'static u8;
    extern "C" fn ref_byte3(&self) -> &'static u8;
    extern "C" fn ref_byte4(&self) -> &'static u8;
    extern "C" fn unk_always_false(&self) -> bool;
    extern "C" fn free_base(this: &&Self, allocator: *const u8);
    extern "C" fn class_size(&self) -> usize;
}

impl DLRuntimeClassTrait for DLRuntimeClass {
    extern "C" fn class_name(&self) -> &'static u8 {
        self._class_name
    }
    extern "C" fn class_name_w(&self) -> &'static u16 {
        self._class_name_w
    }
    extern "C" fn ref_byte1(&self) -> &'static u8 {
        &0
    }
    extern "C" fn ref_byte2(&self) -> &'static u8 {
        &0
    }
    extern "C" fn ref_byte3(&self) -> &'static u8 {
        &0
    }
    extern "C" fn ref_byte4(&self) -> &'static u8 {
        &0
    }

    extern "C" fn unk_always_false(&self) -> bool {
        false
    }

    extern "C" fn free_base(this: &&Self, allocator: *const u8) {
        todo!("{this:?} {allocator:?}")
    }

    extern "C" fn class_size(&self) -> usize {
        std::mem::size_of::<Self>()
    }
}
