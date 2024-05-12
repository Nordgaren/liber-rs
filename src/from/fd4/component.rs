use cstr::cstr;
use widestring::widecstr;

use crate::from::DLRF::DLRuntimeClassType;
use crate::{CppClass, DestructorFn, VTable};

pub type GetRuntimeClassFn<C> =
    extern "C" fn(&CppClass<C>) -> &'static crate::from::DLRF::DLRuntimeClass;

pub type FD4ComponentBase = CppClass<FD4ComponentBaseType>;
const _: () = assert!(std::mem::size_of::<CppClass<FD4ComponentBaseType>>() == 0x8);

#[repr(C)]
pub struct FD4ComponentBaseVTable<C: VTable> {
    /// Get the runtime class object
    ///
    /// * `return`: `DLRF::DLRuntimeClass*` pointer to the runtime class
    pub get_runtime_class: GetRuntimeClassFn<C>,
    pub destructor: DestructorFn<C>,
}
const _: () = assert!(std::mem::size_of::<FD4ComponentBaseVTable<FD4ComponentBaseType>>() == 0x10);

impl<C: VTable> FD4ComponentBaseVTable<C>
where
    CppClass<C>: FD4ComponentBaseTrait,
{
    pub const fn new() -> Self {
        Self {
            get_runtime_class: <CppClass<C> as DLRuntimeClassTrait>::get_runtime_class,
            destructor: <CppClass<C> as FD4ComponentBaseTrait>::destructor,
        }
    }
}

/// Reflection implementation for a given class.
///
/// A class that automatically instantiates reflective DLRF::DLRuntimeClass
/// instances for classes that derive from it. Commonly used in the ELDEN RING
/// codebase.
#[repr(C)]
#[derive(Default)]
pub struct FD4ComponentBaseType;
const _: () = assert!(std::mem::size_of::<FD4ComponentBaseType>() == 0x0);

impl VTable for FD4ComponentBaseType {
    type Table = FD4ComponentBaseVTable<FD4ComponentBaseType>;
    const TABLE: &'static Self::Table = &FD4ComponentBaseVTable {
        get_runtime_class: FD4ComponentBase::get_runtime_class,
        destructor: FD4ComponentBase::destructor,
    };
}

pub trait DLRuntimeClassTrait {
    extern "C" fn get_runtime_class(&self) -> &'static crate::from::DLRF::DLRuntimeClass;
}
impl DLRuntimeClassTrait for FD4ComponentBase {
    extern "C" fn get_runtime_class(&self) -> &'static crate::from::DLRF::DLRuntimeClass {
        static DL_RUNTIME_CLASS: crate::from::DLRF::DLRuntimeClass =
            crate::from::DLRF::DLRuntimeClass::from_data(DLRuntimeClassType::new(
                cstr!("FD4ComponentBase"),
                widecstr!("FD4ComponentBase"),
            ));
        &DL_RUNTIME_CLASS
    }
}

pub trait FD4ComponentBaseTrait: DLRuntimeClassTrait {
    extern "C" fn destructor(&self) {}
}
impl FD4ComponentBaseTrait for FD4ComponentBase {}
