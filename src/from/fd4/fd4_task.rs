use std::ffi::c_void;
use std::ops::Deref;

use cstr::cstr;
use widestring::widecstr;

use crate::from::CS;
use crate::from::DLRF::DLRuntimeClassType;
use crate::from::FD4::time::FD4Time;
use crate::from::FD4::{
    DLRuntimeClassTrait, FD4ComponentBaseTrait, FD4ComponentBaseType, FD4ComponentBaseVTable,
};
use crate::{CppClass, VTable};

pub type ExecuteFn<C> = extern "C" fn(_this: &CppClass<C>, data: &FD4TaskData);

pub type FD4TaskBase = CppClass<FD4TaskBaseType>;
const _: () = assert!(std::mem::size_of::<CppClass<FD4TaskBaseType>>() == 0x10);

#[repr(C)]
pub struct FD4TaskBaseVTable<C: VTable> {
    pub fd4component_base_vtable: FD4ComponentBaseVTable<C>,
    pub execute: ExecuteFn<C>,
}
const _: () = assert!(std::mem::size_of::<FD4TaskBaseVTable<FD4TaskBaseType>>() == 0x18);
impl<C: VTable> FD4TaskBaseVTable<C>
where
    CppClass<C>: FD4TaskBaseTrait,
{
    pub const fn new() -> Self {
        Self {
            fd4component_base_vtable: FD4ComponentBaseVTable::new(),
            execute: <CppClass<C> as FD4TaskBaseTrait>::execute,
        }
    }
}

impl FD4ComponentBaseTrait for FD4TaskBase {}

impl DLRuntimeClassTrait for FD4TaskBase {
    extern "C" fn get_runtime_class(&self) -> &'static crate::from::DLRF::DLRuntimeClass {
        static DL_RUNTIME_CLASS: crate::from::DLRF::DLRuntimeClass =
            crate::from::DLRF::DLRuntimeClass::from_data(DLRuntimeClassType::new(
                cstr!("FD4TaskBase"),
                widecstr!("FD4TaskBase"),
            ));
        &DL_RUNTIME_CLASS
    }
}

impl<C: VTable> Deref for FD4TaskBaseVTable<C> {
    type Target = FD4ComponentBaseVTable<C>;

    fn deref(&self) -> &Self::Target {
        &self.fd4component_base_vtable
    }
}

/// the base task interface.
///
/// Minimal implementation needed for any task.
/// New tasks should derive from CS::CSEzTask instead,
/// as it implements the necessary task registration methods.
#[repr(C)]
pub struct FD4TaskBaseType {
    base: FD4ComponentBaseType,
    unk: *const c_void,
}
const _: () = assert!(std::mem::size_of::<FD4TaskBaseType>() == 0x8);

impl Default for FD4TaskBaseType {
    fn default() -> Self {
        Self {
            base: Default::default(),
            unk: std::ptr::null(),
        }
    }
}

impl Deref for FD4TaskBaseType {
    type Target = FD4ComponentBaseType;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl VTable for FD4TaskBaseType {
    type Table = FD4TaskBaseVTable<FD4TaskBaseType>;
    const TABLE: &'static Self::Table = &FD4TaskBaseVTable::new();
}

pub trait FD4TaskBaseTrait: FD4ComponentBaseTrait {
    extern "C" fn execute(&self, data: &FD4TaskData);
    extern "C" fn get_runtime_class(&self, data: &FD4TaskData) {
        todo!("{data:?}")
    }
    extern "C" fn destructor(&self) {}
}

impl FD4TaskBaseTrait for FD4TaskBase {
    extern "C" fn execute(&self, data: &FD4TaskData) {
        todo!("{data:?}")
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct FD4TaskData {
    time: FD4Time,
    task_group_id: CS::cstgi,
    seed: i32,
}
const _: () = assert!(std::mem::size_of::<FD4TaskData>() == 0x18);
