use std::marker::Copy;
use std::clone::Clone;

#[derive(Copy, Clone)]
pub enum ConstantPoolType {
    Class,
    FieldRef,
    MethodRef,
    InterfaceMethodRef,
    String,
    Integer,
    Float,
    Long,
    Double,
    NameAndType,
    Utf8(u16),
    MethodHandle,
    MethodType,
    Dynamic,
    InvokeDynamic,
    Module,
    Package
}

impl ConstantPoolType {
    pub const fn size_in_slot(&self) -> u16 {
        use ConstantPoolType::*;

        match self {
            Class | Package | Module | MethodType | Utf8(_) | String | MethodHandle | InvokeDynamic | Dynamic | NameAndType | Float | Integer | InterfaceMethodRef | MethodRef | FieldRef => 1,
            Double | Long => 2
        }
    }
    pub fn size_in_header(&self) -> usize {
        use ConstantPoolType::*;

        match self {
            Utf8(len) => (3 + len) as usize,
            Class | Package | Module | MethodType | String => 3,
            MethodHandle => 4,
            InvokeDynamic | Dynamic | NameAndType | Float | Integer | InterfaceMethodRef | MethodRef | FieldRef => 5,
            Double | Long => 9
        }
    }
}