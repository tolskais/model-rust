mod reader;

use reader::{ClassBuffer, ClassReader, ConstantPoolType};

fn main() {
    let buffer = ClassBuffer::from_file("test.class");
    let mut reader = ClassReader::create(&buffer).expect("Could not read constants");

    let access_flag = reader.next_u16();
    let class_name = reader.next_u16();
    let super_name = reader.next_u16();

    let count = reader.next_u16();
    for _ in 0..count {
        reader.skip(ConstantPoolType::Class.size_in_header());
    }

    let count = reader.next_u16();
    for _ in 0..count {
        reader.skip(ConstantPoolType::FieldRef.size_in_header());
    }
    
    let count = reader.next_u16();
    println!("{count}");
    for _ in 0..count {
        let access_flag = reader.next_u16();
        let method_name = reader.next_utf8();
        let method_desc = reader.next_utf8();
        println!("{}{}", method_name, method_desc);
    }
}
