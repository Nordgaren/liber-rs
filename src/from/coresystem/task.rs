use crate::from::CS::taskgroups::CSTaskGroup;
use crate::from::FD4::{DLRuntimeClass, FD4ComponentBaseClass, FD4TaskBaseClass, FD4TaskBaseType, FD4TaskBaseVTable, FD4TaskData};
use crate::{CppClass, VTable};
use std::ops::Deref;
use cstr::cstr;
use widestring::widecstr;
use crate::from::DLRF::DLRuntimeClassType;

/// Typedef of a special unsigned integer type that may represent a task id.
///
/// See the CS_TASK_GROUP_ID macro in fd4.task.hpp
#[allow(non_camel_case_types)]
pub type cstgi = u32;

/// Virtual method that is called when a CS::CSEzTask is executed.
///
/// Implement this method in a custom task class to provide a
/// callback for when the task is executed.
///
/// # Arguments
///
/// * `data`:  a struct with additional data passed to the task, like delta time and the task group
pub type EztaskExecuteFn<C> = extern "C" fn(_this: &CppClass<C>, data: &FD4TaskData);
/// Register a task to be called in a specified task group.
///
/// Call this method with a task instance to register it for execution.
/// ELDEN RING task runners execute task groups in a strict order, going
/// from FrameBegin to FrameEnd. After a task is registered, it will be
/// executed in the next pass of all the task groups the following frame,
/// calling its eztask_execute method.
///
/// # Arguments
///
/// * `task_group`: a value from the CS::CSTaskGroup enum when the task should be executed
pub type RegisterTaskFn<C> = extern "C" fn(_this: &CppClass<C>, task_group: CSTaskGroup);
/// Free the task, suspending its execution after it has been
/// registered.
///
/// Call this method to stop executing a task. It may still execute on the
/// current pass of the task groups, but will not execute on the next.
/// Keep the task lifetime @link CS::CSEzTask disclaimer @endlink in mind
/// when freeing or destroying a task.
pub type FreeTaskFn<C> = extern "C" fn(_this: &CppClass<C>);

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

/// Inherit from this minimal task interface to create a custom task.
///
/// Used by ELDEN RING to queue and free asynchronous tasks in the task queue.
/// Once a task is registered, it runs every frame when the task group specified
/// to the register_task call is executed. The task group execution order
/// is strict with regard to other task groups, but individual task execution
/// order *inside a task group* is unspecified.
///
/// # warning
/// Disclaimer: a task instance must not go out of scope as long as it
/// is registered and executing. Use from::unique_ptr from from_unique_ptr.hpp to
/// correctly manage its lifetime. Destroying it before it has executed on this
/// pass will leave a dangling pointer in the task queue.
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
    extern "C" fn free_task(&self) {
        todo!()
    }
}

#[repr(C, packed(1))]
pub struct CSEzTaskProxy {
    owner: *const CSEzTaskType,
    task_group: CSTaskGroup,
}
const _: () = assert!(std::mem::size_of::<CSEzTaskProxy>() == 0xC);
const _: () = assert!(std::mem::size_of::<CSTaskGroup>() == 0x4);

impl CSEzTaskProxy {
    pub fn get_task_group(&self) -> CSTaskGroup {
        self.task_group
    }
}
