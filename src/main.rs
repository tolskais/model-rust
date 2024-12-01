use std::fs;
use std::ops::Index;

type Start = u8;
type Length = u16;

const MAGIC_SIZE: u8 = 4;
const MINOR_SIZE: u8 = 2;
const MAJOR_SIZE: u8 = 2;
const CONSTANT_SIZE: u8 = 2;

enum ConstantPoolType {
    Class(Start),
    FieldRef(Start),
    MethodRef(Start),
    InterfaceMethodRef(Start),
    String(Start),
    Integer(Start),
    Float(Start),
    Long(Start),
    Double(Start),
    NameAndType(Start),
    Utf8(Start, Length),
    MethodHandle(Start),
    MethodType(Start),
    Dynamic(Start),
    InvokeDynamic(Start),
    Module(Start),
    Package(Start)
}

impl ConstantPoolType {
    fn size_in_slot(&self) -> u8 {
        use ConstantPoolType::*;

        match self {
            Class(_) | Package(_) | Module(_) | MethodType(_) | Utf8(_, _) | String(_) | MethodHandle(_) | InvokeDynamic(_) | Dynamic(_) | NameAndType(_) | Float(_) | Integer(_) | InterfaceMethodRef(_) | MethodRef(_) | FieldRef(_) => 1,
            Double(_) | Long(_) => 2
        }
    }
    fn size_in_header(&self) -> u8 {
        use ConstantPoolType::*;

        match self {
            Class(_) | Package(_) | Module(_) | MethodType(_) | Utf8(_, _) | String(_) => 3,
            MethodHandle(_) => 4,
            InvokeDynamic(_) | Dynamic(_) | NameAndType(_) | Float(_) | Integer(_) | InterfaceMethodRef(_) | MethodRef(_) | FieldRef(_) => 5,
            Double(_) | Long(_) => 9
        }
    }
}

struct ClassBuffer {
    buf: Vec<u8>
}

impl Index<u16> for ClassBuffer {
    type Output = u8;
    
    fn index(&self, index: u16) -> &Self::Output {
        &self.buf[index as usize]
    }
}


impl ClassBuffer {
    fn read(filename: &str) -> Self {
        Self {
            buf: fs::read(filename)
            .expect("asdf")
        }
    }

    // Assume little endians
    fn read_u16(&self, index: u16) -> u16 {
        ((self[index] as u16) << 8) | (self[index + 1] as u16)
    }

    fn read_constants(&self) -> Result<Vec<ConstantPoolType>, &str> {
        use ConstantPoolType::*;

        let mut count = self.read_u16((MAGIC_SIZE + MAJOR_SIZE + MINOR_SIZE).into());
        println!("# of constants: {count}");

        let mut constants: Vec<ConstantPoolType> = Vec::with_capacity(count as usize);

        // https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-4.html
        let mut index = MAGIC_SIZE + MAJOR_SIZE + MINOR_SIZE + CONSTANT_SIZE;
        while count > 0 {
            let constant_type = self[index.into()];
            let item = match constant_type {
                7 => Class(index),
                9 => FieldRef(index),
                10 => MethodRef(index),
                11 => InterfaceMethodRef(index),
                8 => String(index),
                3 => Integer(index),
                4 => Float(index),
                5 => Long(index),
                6 => Double(index),
                12 => NameAndType(index),
                1 => {
                    let size = self.read_u16((index + 1).into());
                    let ret = Utf8(index, size);
                    index += u8::try_from(size).expect("The constant pool contains an unexpectly long utf8 constant.");
                    ret
                },
                15 => MethodHandle(index),
                16 => MethodType(index),
                17 => Dynamic(index),
                18 => InvokeDynamic(index),
                19 => Module(index),
                20 => Package(index),
                _ => return Err("Could not read the constant")
            };

            index += item.size_in_header();
            count -= u16::from(item.size_in_slot());

            println!("{index}: {count}");

            constants.push(item);
        }

        return Ok(constants);
    }
}

fn main() {
    let buffer = ClassBuffer::read("test.class");
    let constants = buffer.read_constants();
    
}
