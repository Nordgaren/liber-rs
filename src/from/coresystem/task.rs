use crate::from::CS::taskgroups::CSTaskGroup;
use crate::from::FD4::{DLRuntimeClass, FD4ComponentBaseClass, FD4TaskBaseClass, FD4TaskBaseType, FD4TaskBaseVTable, FD4TaskData};
use crate::{CppClass, VTable};
use std::ops::Deref;
use cstr::cstr;
use widestring::widecstr;
use crate::from::DLRF::DLRuntimeClassType;

#[allow(non_camel_case_types)]
pub type cstgi = u32;

pub type EztaskExecuteFn<C> = extern "C" fn(_this: &CppClass<C>, data: &FD4TaskData);
pub type RegisterTaskFn<C> = extern "C" fn(_this: &CppClass<C>, task_group: CSTaskGroup);
pub type FreeTaskFn<C> = extern "C" fn(_this: &CppClass<C>, task_group: CSTaskGroup);

#[repr(C)]
pub struct CSEzTaskVTable<C: VTable> {
    pub fd4task_base_vtable: FD4TaskBaseVTable<C>,
    pub eztask_execute: EztaskExecuteFn<C>,
    pub register_task: RegisterTaskFn<C>,
    pub free_task: FreeTaskFn<C>,
}
const _: () = assert!(std::mem::size_of::<CSEzTaskVTable<CSEzTaskType>>() == 0x30);

impl<C: VTable> Deref for CSEzTaskVTable<C> {
    type Target = FD4TaskBaseVTable<C>;

    fn deref(&self) -> &Self::Target {
        &self.fd4task_base_vtable
    }
}

pub type CSEzTask = CppClass<CSEzTaskType>;
//const _: () = assert!(std::mem::size_of::<CSEzTask>() == 0x1C);

impl CSEzTaskClass for CSEzTask {}

#[repr(C)]
pub struct CSEzTaskType {
    fd4_task_base: FD4TaskBaseType,
    proxy: CSEzTaskProxy,
}
//const _: () = assert!(std::mem::size_of::<CSEzTaskType>() == 0x14);

impl CSEzTaskType {
    pub fn new(task_group: CSTaskGroup) -> Self {
        let mut task = Self {
            fd4_task_base: Default::default(),
            proxy: CSEzTaskProxy {
                owner: std::ptr::null(),
                task_group,
            },
        };
        task.proxy.owner = &task as *const CSEzTaskType;
        task
    }
}

impl Default for CSEzTaskType {
    fn default() -> Self {
        let mut task = Self {
            fd4_task_base: Default::default(),
            proxy: CSEzTaskProxy {
                owner: std::ptr::null(),
                task_group: CSTaskGroup::INVALID,
            },
        };
        task.proxy.owner = &task as *const CSEzTaskType;
        task
    }
}
impl Deref for CSEzTaskType {
    type Target = FD4TaskBaseType;

    fn deref(&self) -> &Self::Target {
        &self.fd4_task_base
    }
}

impl VTable for CSEzTaskType {
    type Table = CSEzTaskVTable<CSEzTaskType>;
    const TABLE: &'static Self::Table = &CSEzTaskVTable::new();
}

impl<C: VTable> CSEzTaskVTable<C>
    where
        CppClass<C>: CSEzTaskClass,
{
    pub const fn new() -> Self {
        Self  {
            fd4task_base_vtable: FD4TaskBaseVTable::new(),
            eztask_execute: <CppClass<C> as CSEzTaskClass>::eztask_execute,
            register_task: <CppClass<C> as CSEzTaskClass>::register_task,
            free_task: <CppClass<C> as CSEzTaskClass>::free_task,
        }
    }
}
impl DLRuntimeClass for CSEzTask {
    extern "C" fn get_runtime_class(&self) -> &'static crate::from::DLRF::DLRuntimeClass {
        static DL_RUNTIME_CLASS: crate::from::DLRF::DLRuntimeClass =
            crate::from::DLRF::DLRuntimeClass::new(DLRuntimeClassType::new(
                cstr!("CSEzTask"),
                widecstr!("CSEzTask"),
            ));
        &DL_RUNTIME_CLASS
    }
}

impl FD4TaskBaseClass for CSEzTask {
    extern "C" fn execute(&self, data: &FD4TaskData) {
        todo!("{data:?}")
    }
}
impl FD4ComponentBaseClass for CSEzTask {}

pub trait CSEzTaskClass: FD4TaskBaseClass {
    extern "C" fn eztask_execute(&self, data: &FD4TaskData) {
        self.execute(data)
    }
    extern "C" fn register_task(&self, task_group: CSTaskGroup) {
        todo!("{:?}", task_group)
    }
    extern "C" fn free_task(&self, task_group: CSTaskGroup) {
        todo!("{:?}", task_group)
    }
}

#[repr(C, packed(1))]
pub struct CSEzTaskProxy {
    owner: *const CSEzTaskType,
    task_group: CSTaskGroup,
}
const _: () = assert!(std::mem::size_of::<CSEzTaskProxy>() == 0xC);
const _: () = assert!(std::mem::size_of::<CSTaskGroup>() == 0x4);
