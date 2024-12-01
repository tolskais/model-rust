mod constant;

pub use self::constant::ConstantPoolType;
use core::str;
use std::{fs, ops::Index};

struct Constant {
    value_type: constant::ConstantPoolType,
    index: u16
}

// ClassFile structure
// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.1
const MAGIC_SIZE: u16 = 4;
const MINOR_SIZE: u16 = 2;
const MAJOR_SIZE: u16 = 2;
const CONSTANT_SIZE: u16 = 2;
// Pool Constant
const ACCESS_FLAG_SIZE: u16 = 2;
const CLASS_SIZE: u16 = 2;
const SUPER_SIZE: u16 = 2;
const ITF_COUNT_SIZE: u16 = 2;
// Interfaces
const FIELD_COUNT_SIZE: u16 = 2;
// Fields
const METHOD_COUNT: u16 = 2;
// Methods
const ATTRIBUTE_COUNT: u16 = 2;
// Attributes

pub struct ClassBuffer {
    buf: Vec<u8>
}

impl Index<usize> for ClassBuffer {
    type Output = u8;
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[index]
    }
}

impl ClassBuffer {
    pub fn from_file(filename: &str) -> Self {
        let buf = fs::read(filename).expect("Cannot find the class file");
        Self { buf }
    }

    pub fn read_u16(&self, index: usize) -> u16 {
        ((self[index] as u16) << 8) | (self[index + 1] as u16)
    }

    pub fn read_utf8(&self, def: &Constant) -> Result<&str, &str> {
        match def.value_type {
            constant::ConstantPoolType::Utf8(len) => {
                let start = def.index as usize + 3;
                let end = start + len as usize;
                Result::Ok(str::from_utf8(&self.buf[start..end]).expect("why"))
            },
            _ => Result::Err("what")
        }
    }
}

pub struct ClassReader<'a> {
    buf: &'a ClassBuffer,
    constants: Vec<Constant>,
    index: usize
}

impl <'a> ClassReader<'a> {
    pub fn skip(&mut self, size: usize) {
        self.index += size;
    }

    pub fn next_u16(&mut self) -> u16 {
        let current = self.index;
        self.index += 2;
        self.buf.read_u16(current)
    }

    pub fn next_utf8(&mut self) -> &'a str {
        let constant_index = self.next_u16();
        let constant = self.constants.get(constant_index as usize - 1).expect("Could not read a constant");
        self.buf.read_utf8(constant).expect("Invalid utf8 string")
    }

    fn read_constant(buf: &ClassBuffer, index: usize) -> Result<ConstantPoolType, &'static str> {
        use ConstantPoolType::*;

        match buf[index] {
            7 => Ok(Class),
            9 => Ok(FieldRef),
            10 => Ok(MethodRef),
            11 => Ok(InterfaceMethodRef),
            8 => Ok(String),
            3 => Ok(Integer),
            4 => Ok(Float),
            5 => Ok(Long),
            6 => Ok(Double),
            12 => Ok(NameAndType),
            1 => {
                let size = buf.read_u16(index + 1);
                Ok(Utf8(size))
            },
            15 => Ok(MethodHandle),
            16 => Ok(MethodType),
            17 => Ok(Dynamic),
            18 => Ok(InvokeDynamic),
            19 => Ok(Module),
            20 => Ok(Package),
            _ => Err("Unknown constant type")
        }
    }

    pub fn create(buf: &ClassBuffer) -> Result<ClassReader, &'static str> {
        const CONSTANT_START_INDEX: u16 = MAGIC_SIZE + MINOR_SIZE + MAJOR_SIZE;
        let mut count = buf.read_u16(CONSTANT_START_INDEX as usize) - 1;
        println!("# of constants: {count}");

        let mut constants: Vec<Constant> = Vec::with_capacity(count as usize);

        // https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-4.html
        let mut item_index = (CONSTANT_START_INDEX + CONSTANT_SIZE) as usize;
        while count > 0 {
            let value_type = Self::read_constant(buf, item_index).expect("");
            let index = item_index as u16;

            constants.push(Constant {
                value_type,
                index
            });

            item_index += value_type.size_in_header();
            count -= value_type.size_in_slot();
        }

        return Ok(ClassReader {
            buf, constants, index: item_index
        });
    }
}