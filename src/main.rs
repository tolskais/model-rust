use core::str;
use std::fs;
use std::ops::Index;

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

struct Constant {
    value_type: ConstantPoolType,
    index: u16
}

enum ConstantPoolType {
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
    fn size_in_slot(&self) -> u16 {
        use ConstantPoolType::*;

        match self {
            Class | Package | Module | MethodType | Utf8(_) | String | MethodHandle | InvokeDynamic | Dynamic | NameAndType | Float | Integer | InterfaceMethodRef | MethodRef | FieldRef => 1,
            Double | Long => 2
        }
    }
    fn size_in_header(&self) -> usize {
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

struct ClassReader {
    buf: Vec<u8>
}

impl Index<usize> for ClassReader {
    type Output = u8;
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[index]
    }
}


impl ClassReader {
    fn from_file(filename: &str) -> Self {
        Self {
            buf: fs::read(filename)
            .expect("asdf")
        }
    }

    fn read_u16(&self, index: usize) -> u16 {
        ((self[index] as u16) << 8) | (self[index + 1] as u16)
    }

    fn read_utf8(&self, def: &Constant) -> Result<&str, &str> {
        match def.value_type {
            ConstantPoolType::Utf8(len) => {
                let start = def.index as usize + 3;
                let end = start + len as usize;
                Result::Ok(str::from_utf8(&self.buf[start..end]).expect("why"))
            },
            _ => Result::Err("what")
        }
    }

    fn read_constants(&self) -> Result<(Vec<Constant>, usize), &str> {
        use ConstantPoolType::*;

        const CONSTANT_START_INDEX: u16 = MAGIC_SIZE + MINOR_SIZE + MAJOR_SIZE;
        let mut count = self.read_u16(CONSTANT_START_INDEX as usize) - 1;
        println!("# of constants: {count}");

        let mut constants: Vec<Constant> = Vec::with_capacity(count as usize);

        // https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-4.html
        let mut item_index = (CONSTANT_START_INDEX + CONSTANT_SIZE) as usize;
        while count > 0 {
            let constant_type = self[item_index];
            let value_type = match constant_type {
                7 => Class,
                9 => FieldRef,
                10 => MethodRef,
                11 => InterfaceMethodRef,
                8 => String,
                3 => Integer,
                4 => Float,
                5 => Long,
                6 => Double,
                12 => NameAndType,
                1 => {
                    let size = self.read_u16(item_index + 1);
                    Utf8(size)
                },
                15 => MethodHandle,
                16 => MethodType,
                17 => Dynamic,
                18 => InvokeDynamic,
                19 => Module,
                20 => Package,
                _ => return Err("Unknown constant type: {}")
            };

            let index = item_index as u16;
            item_index += value_type.size_in_header();
            count -= value_type.size_in_slot();

            constants.push(Constant {
                value_type,
                index
            });            
        }

        return Ok((constants, item_index));
    }
}

fn main() {
    let reader = ClassReader::from_file("test.class");
    let (constants, end_index) = reader.read_constants().expect("Could not parse the header");
    
    let interface_index = end_index + (ACCESS_FLAG_SIZE + CLASS_SIZE + SUPER_SIZE) as usize;
    let interface_count = reader.read_u16(interface_index);
    let field_index = interface_index + ITF_COUNT_SIZE as usize + (ConstantPoolType::Class.size_in_header() * interface_count as usize);
    let field_count = reader.read_u16(field_index);
    let method_index = field_index + FIELD_COUNT_SIZE as usize + (ConstantPoolType::FieldRef.size_in_header() * field_count as usize);
    let method_count = reader.read_u16(method_index);

    let method_name_index = reader.read_u16(method_index + METHOD_COUNT as usize + 2) as usize;
    println!("{method_name_index}");
    let name_constant = constants.get(method_name_index - 1).expect("Could not read the method name");
    let method_name = reader.read_utf8(name_constant).expect("???");
    println!("{}", method_name);
}
