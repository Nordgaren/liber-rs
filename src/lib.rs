use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

pub mod from;

pub use cstr::cstr;
pub use from::details::symbols::*;
pub use inhert_macros_derive::CSEzTask;
pub use widestring::widecstr;

pub type DestructorFn<C> = extern "C" fn(&CppClass<C>);

pub trait VTable
where
    <Self as VTable>::Table: 'static,
{
    type Table;
    const TABLE: &'static Self::Table;
}
#[repr(C)]
pub struct CppClass<C: VTable> {
    pub vtable: &'static C::Table,
    data: C,
}

impl<C: VTable> CppClass<C> {
    pub const fn from_data(data: C) -> Self {
        Self {
            vtable: C::TABLE,
            data,
        }
    }
}

impl<C: VTable + Debug> Debug for CppClass<C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl<C: VTable> Deref for CppClass<C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<C: VTable> DerefMut for CppClass<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
